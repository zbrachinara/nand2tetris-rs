use super::error::ModelConstructionError;
use super::native::Router;
use super::{builtin, Chip, Id};
use crate::channel_range::ChannelRange;
use crate::model::parser::{Chip as ChipRepr, Connection, Form, Interface, Symbol};
use bitvec::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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
}

#[derive(Debug)]
enum Hook {
    Output(Id, ChannelRange),
    Input(Id, ChannelRange),
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum OuterHook {
    Output(ChannelRange),
    Input(ChannelRange),
}

impl IncompleteBarrier {
    fn new(info: ChipInfo) -> Self {
        // generates a buffer and sets it to the proper size
        let storage_size = info.interface.size_in();
        let mut storage = BitVec::with_capacity(storage_size);
        unsafe {
            storage.set_len(storage_size);
        }
        storage.set_uninitialized(false);

        Self {
            chip: info.chip,
            interface: info.interface,
            router: Router { map: Vec::new() },
            default: storage,
        }
    }
}

impl ChipBuilder {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
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
        let mut id_provider = Id(0);
        let out_id = id_provider.next();

        let chips = connections
            .into_iter()
            .map(|conn| {
                Ok((
                    id_provider.next(),
                    (
                        IncompleteBarrier::new(
                            self.registered
                                .get(*(conn.chip_name))
                                .ok_or(ModelConstructionError::Needs(conn.chip_name.to_string()))?
                                .clone(),
                        ),
                        conn.inputs,
                    ),
                ))
            })
            .try_collect::<HashMap<_, _>>()?;

        let mut connection_map = HashMap::new();
        let mut outer_connection_map = HashMap::new();

        // pass one: register all connections
        for (id, (IncompleteBarrier { interface, .. }, inputs)) in chips.iter() {
            for arg in inputs {
                println!("working on {}", arg.internal);
                let Ok(internal_bus) = interface
                    .real_range(*(arg.internal), arg.internal_bus.as_ref()) else {
                        return Err(ModelConstructionError::PinNotFound(
                            arg.internal.to_string(),
                            top_interface.name.clone(),
                        ));
                    };
                let Symbol::Name(external) = arg.external else {
                    // discard all by-value assignments
                    return Err(ModelConstructionError::ValuesNotSupported(
                        arg.internal.to_string(),
                    ))
                };
                let external_bus = top_interface
                    .real_range(*external, arg.external_bus.as_ref())
                    .ok();

                if let Some(outer_range) = external_bus {
                    let (k, hook) = if interface.is_input(*arg.internal) {
                        (
                            OuterHook::Input(outer_range),
                            Hook::Input(id.clone(), internal_bus),
                        )
                    } else {
                        (
                            OuterHook::Output(outer_range),
                            Hook::Output(id.clone(), internal_bus),
                        )
                    };
                    push_to_entry(outer_connection_map.entry(k), hook);
                } else {
                    push_to_entry(
                        connection_map.entry(external.to_string()),
                        if interface.is_input(*arg.internal) {
                            Hook::Input(id.clone(), internal_bus)
                        } else {
                            Hook::Output(id.clone(), internal_bus)
                        },
                    );
                };
            }
        }

        println!("outer connections: {outer_connection_map:?}");
        println!("inner connections: {connection_map:?}");
        unimplemented!("pass two: write back connections");
        unimplemented!("package into chip");
    }
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
    use tap::Tap;

    fn print_test(path: impl AsRef<Path>) {
        let mut builder = ChipBuilder::new().tap_mut(|x| x.with_builtins());
        let file = std::fs::read_to_string(path).unwrap();
        let code = crate::model::parser::create_chip(Span::from(file.as_str())).unwrap();
        builder.register_hdl(code).unwrap();
    }

    #[test]
    fn print_test_not() {
        print_test("../test_files/01/Not.hdl")
    }

    #[test]
    fn print_test_and() {
        print_test("../test_files/01/And.hdl");
    }
}
