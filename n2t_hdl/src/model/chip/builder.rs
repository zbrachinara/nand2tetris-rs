use super::error::ModelConstructionError;
use super::native::{Barrier, Router};
use super::{builtin, Chip, Id};
use crate::channel_range::ChannelRange;
use crate::model::chip::native::{Hook, NativeChip};
use crate::model::parser::{Argument, Chip as ChipRepr, Connection, Form, Interface, Symbol};
use bitvec::prelude::*;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use tap::Tap;

pub struct ChipBuilder {
    registered: HashMap<String, ChipInfo>,
}

pub struct ChipInfo {
    pub interface: Interface,
    pub chip: Box<dyn Chip>,
}

impl Clone for ChipInfo {
    fn clone(&self) -> Self {
        Self {
            interface: self.interface.clone(),
            chip: self.chip.boxed_clone(),
        }
    }
}

struct IncompleteBarrier {
    chip: Box<dyn Chip>,
    interface: Interface,
    router: Router,
    default: BitVec,
    clock_mask: BitVec,
}

#[derive(Debug)]
struct EdgeSet {
    output: Option<Hook>,
    inputs: Vec<Hook>,
}

impl EdgeSet {
    fn insert(
        entry: Entry<String, Self>,
        item: Hook,
        as_input: bool,
    ) -> Result<(), ModelConstructionError> {
        let mut conflict = false;
        let key = entry.key().clone();

        entry
            .and_modify(|mut entry| {
                if as_input {
                    entry.inputs.push(item.clone());
                } else if let Some(_) = entry.output {
                    conflict = true;
                } else {
                    entry.output = Some(item.clone());
                }
            })
            .or_insert_with(|| {
                if as_input {
                    EdgeSet {
                        output: None,
                        inputs: vec![item],
                    }
                } else {
                    EdgeSet {
                        output: Some(item),
                        inputs: vec![],
                    }
                }
            });

        if conflict {
            Err(ModelConstructionError::ConflictingSources(key))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum OuterHook {
    Output(ChannelRange),
    Input(ChannelRange),
}

impl IncompleteBarrier {
    fn new(info: ChipInfo) -> Self {
        let storage_size = info.interface.size_in();
        Self {
            chip: info.chip,
            interface: info.interface,
            router: Router { map: Vec::new() },
            default: BitVec::repeat(false, storage_size),
            clock_mask: BitVec::repeat(false, storage_size),
        }
    }

    fn complete(self) -> Barrier {
        Barrier {
            in_buffer: self.default.clone(),
            intermediate: self.default.clone(),
            clock_mask: self.clock_mask,
            out_buffer: self.default.clone().tap_mut(|v| v.fill(false)),
            chip: self.chip,
            router: self.router,
        }
    }
}

impl ChipBuilder {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn get_registry(&self) -> &HashMap<String, ChipInfo> {
        &self.registered
    }

    pub fn get_chip_info(&self, name: &str) -> Option<ChipInfo> {
        self.registered.get(name).map(|c| c.clone())
    }

    pub fn with_builtins(&mut self) {
        self.registered
            .extend([("Nand".to_string(), builtin::nand())]);
    }

    pub fn register_hdl(&mut self, chip: ChipRepr) -> Result<(), ModelConstructionError> {
        let top_interface = chip.interface();
        let ChipRepr { name, logic, .. } = chip;

        if let Form::Native(conns) = logic {
            if self
                .registered
                .insert(name.to_string(), self.build_native(top_interface, conns)?)
                .is_some()
            {
                Err(ModelConstructionError::Rebuilt(name.to_string()))
            } else {
                Ok(())
            }
        } else {
            panic!("dynamic loading of native chips is not yet supported")
        }
    }

    fn build_native(
        &self,
        top_interface: Interface,
        connections: Vec<Connection>,
    ) -> Result<ChipInfo, ModelConstructionError> {
        println!("building chip {}", top_interface.name);

        let mut id_provider = Id(0);
        let out_id = id_provider.next();

        let (mut chips, needed): (HashMap<_, _>, Vec<_>) = connections
            .into_iter()
            .map(|conn| {
                Ok((
                    id_provider.next(),
                    (
                        IncompleteBarrier::new(
                            self.registered
                                .get(*(conn.chip_name))
                                .ok_or(conn.chip_name.to_string())?
                                .clone(),
                        ),
                        conn.inputs,
                    ),
                ))
            })
            .partition_result();

        if !needed.is_empty() {
            return Err(ModelConstructionError::Needs(needed));
        }

        let (conn_map, outer_conn_map) = create_connection_maps(&mut chips, &top_interface)?;

        println!("outer connections: {outer_conn_map:?}");
        println!("inner connections: {conn_map:?}");

        // pass two: write back connections
        let mut in_router = Router::new();
        // write back top level connections
        for (outer_hook, hooks) in outer_conn_map {
            for hook in hooks {
                if let OuterHook::Input(input_range) = outer_hook {
                    in_router.add_hook(input_range, hook);
                } else if let OuterHook::Output(output_range) = outer_hook {
                    chips.get_mut(&hook.id).unwrap().0.router.add_hook(
                        hook.range,
                        Hook {
                            id: out_id.clone(),
                            range: output_range,
                        },
                    );
                }
            }
        }
        // then write back internal connections
        for (k, EdgeSet { output, inputs }) in conn_map {
            let output = output.ok_or(ModelConstructionError::NoSource(k))?;
            for input in inputs {
                chips
                    .get_mut(&output.id)
                    .unwrap()
                    .0
                    .router
                    .add_hook(output.range, input)
            }
        }

        let registry = chips
            .into_iter()
            .map(|(id, (barrier, _))| (id, barrier.complete()))
            .collect::<HashMap<_, _>>();
        let out_buffer = BitVec::repeat(false, top_interface.size_out());

        Ok(ChipInfo {
            interface: top_interface,
            chip: Box::new(NativeChip {
                registry,
                in_router,
                out_chip: out_id,
                out_buffer,
                request_queue: VecDeque::new(),
            }),
        })
    }
}

fn create_connection_maps(
    chips: &mut HashMap<Id, (IncompleteBarrier, Vec<Argument>)>,
    top_interface: &Interface,
) -> Result<(HashMap<String, EdgeSet>, HashMap<OuterHook, Vec<Hook>>), ModelConstructionError> {
    let mut connection_map = HashMap::new();
    let mut outer_connection_map = HashMap::new();

    // pass one: register all connections
    for (
        id,
        (
            IncompleteBarrier {
                interface, default, ..
            },
            inputs,
        ),
    ) in chips.iter_mut()
    {
        for arg in inputs {
            let Ok(internal_bus) = interface
                .real_range(*(arg.internal), arg.internal_bus.as_ref()) else {
                    return Err(ModelConstructionError::PinNotFound(
                        arg.internal.to_string(),
                        top_interface.name.clone(),
                    ));
                };
            match arg.external {
                Symbol::Name(external) => {
                    let external_bus = top_interface
                        .real_range(*external, arg.external_bus.as_ref())
                        .ok();
                    let hook = Hook {
                        id: id.clone(),
                        range: internal_bus,
                    };

                    if let Some(outer_range) = external_bus {
                        let k = if interface.is_input(*arg.internal) {
                            OuterHook::Input
                        } else {
                            OuterHook::Output
                        }(outer_range);

                        push_to_entry(outer_connection_map.entry(k), hook);
                    } else {
                        EdgeSet::insert(
                            connection_map.entry(external.to_string()),
                            hook,
                            interface.is_input(*arg.internal),
                        )?;
                    };
                }
                Symbol::Value(ref val) => {
                    let range = interface
                        .real_range(*arg.internal, arg.internal_bus.as_ref())
                        .unwrap() // TODO propogate error, don't unwrap
                        .as_range();
                    default[range].fill((*val).into());
                    #[cfg(test)]
                    println!("modified default into {default:?}");
                }
                Symbol::Number(_) => {
                    return Err(ModelConstructionError::ValuesNotSupported(
                        arg.internal.to_string(),
                    ));
                }
            }
        }
    }

    Ok((connection_map, outer_connection_map))
}

fn push_to_entry<K, T>(entry: Entry<K, Vec<T>>, value: T) {
    match entry {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(value);
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![value]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Span;
    use std::path::Path;

    fn print_test(builder: &mut ChipBuilder, path: impl AsRef<Path>) {
        print_test_str(builder, std::fs::read_to_string(path).unwrap().as_str());
    }

    fn print_test_str(builder: &mut ChipBuilder, code: &str) {
        builder.with_builtins();
        let code = crate::model::parser::create_chip(Span::from(code)).unwrap();
        builder.register_hdl(code).unwrap();
    }

    #[test]
    fn print_test_not() {
        print_test(&mut ChipBuilder::new(), "../test_files/01/Not.hdl")
    }

    #[test]
    fn print_test_and() {
        let mut builder = ChipBuilder::new();
        print_test(&mut builder, "../test_files/01/And.hdl");
        print_test(&mut builder, "../test_files/01/And16.hdl")
    }

    #[test]
    fn print_test_boolean() {
        let mut builder = ChipBuilder::new();
        print_test(&mut builder, "../test_files/01/And.hdl");

        let code = r#"
CHIP Code {
    IN a;
    OUT b;

    PARTS:
    And(a=a, b=true, out=b);
}
        "#;

        print_test_str(&mut builder, code);
    }
}
