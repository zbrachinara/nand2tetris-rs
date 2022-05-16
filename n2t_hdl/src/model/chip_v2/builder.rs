use super::error::ModelConstructionError;
use super::native::Router;
use super::{builtin, Chip, Id};
use crate::channel_range::ChannelRange;
use crate::model::parser::{Channel, Connection, Interface, Symbol};
use crate::model::parser::{Chip as ChipRepr, Form};
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
    Output(Id),
    Input(Id),
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
        let ChipRepr {
            name,
            in_pins,
            out_pins,
            logic,
        } = chip;
        let top_router: Vec<(ChannelRange, (Id, ChannelRange))> = Vec::new();

        if let Form::Native(conns) = logic {
            if self
                .registered
                .insert(
                    name.to_string(),
                    self.build_native(*name, in_pins, out_pins, conns)?,
                )
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
        name: &str,
        in_pins: Vec<Channel>,
        out_pins: Vec<Channel>,
        connections: Vec<Connection>,
    ) -> Result<ChipInfo, ModelConstructionError> {
        let mut id_provider = Id(0);
        let out_id = id_provider.next();

        let mut connection_map: HashMap<String, (Vec<Hook>, usize)> = HashMap::new();

        let chips = connections
            .into_iter()
            .map(|conn| {
                Result::<_, ModelConstructionError>::Ok((
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

        // pass one: register all connections
        for (id, (IncompleteBarrier { interface, .. }, inputs)) in chips.iter() {
            for arg in inputs {
                let hook = if interface.is_input(*arg.internal) {
                    Hook::Input(id.clone())
                } else {
                    Hook::Output(id.clone())
                };

                if let Symbol::Name(external) = arg.external {
                    let internal_bus_size = arg.internal_bus.map(|x| x.size()).unwrap_or(1);
                    match connection_map.entry(external.to_string()) {
                        Entry::Occupied(entry) => {
                            if entry.get().1 == internal_bus_size {
                                println!("Inserting {hook:?} into {external}");
                                Ok(entry.into_mut().0.push(hook))
                            } else {
                                Err(ModelConstructionError::MismatchedSizes {
                                    failed: arg.internal.to_string(),
                                    expected: entry.get().1,
                                    actual: internal_bus_size,
                                })
                            }?;
                        }
                        Entry::Vacant(entry) => {
                            println!("Inserting {hook:?} into {external}");
                            entry.insert((vec![hook], internal_bus_size));
                        }
                    };
                } else {
                    // discard all by-value assignments
                    return Err(ModelConstructionError::ValuesNotSupported(
                        arg.internal.to_string(),
                    ));
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Span;
    use tap::Tap;

    #[test]
    fn print_test_not() {
        let mut builder = ChipBuilder::new().tap_mut(|x| x.with_builtins());
        let file = std::fs::read_to_string("../test_files/01/Not.hdl").unwrap();
        let code = crate::model::parser::create_chip(Span::from(file.as_str())).unwrap();
        builder.register_hdl(code).unwrap();
    }
}
