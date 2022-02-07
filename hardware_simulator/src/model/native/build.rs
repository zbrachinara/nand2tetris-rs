use crate::bus_range::BusRange;
use crate::model::parser::{self, Argument, Interface, Symbol};
use crate::model::Chip;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EdgeSet {
    pub input: Option<Endpoint>,
    pub outputs: Vec<Endpoint>,
}

impl EdgeSet {
    fn new_with(endpoint: Endpoint, as_input: bool) -> Result<Self, ()> {
        let mut new = EdgeSet {
            input: None,
            outputs: Vec::new(),
        };
        new.add(endpoint, as_input)?;
        Ok(new)
    }

    fn add(&mut self, endpoint: Endpoint, as_input: bool) -> Result<(), ()> {
        if as_input {
            if matches!(self.input, Some(_)) {
                return Err(());
            } else {
                self.input = Some(endpoint)
            }
        } else {
            self.outputs.push(endpoint)
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Endpoint {
    pub index: usize,
    pub range: BusRange,
}

pub fn edges_from_connections(
    conn_names: &Vec<parser::Connection>,
    dependents: &Vec<Box<dyn Chip>>,
) -> HashMap<String, EdgeSet> {
    let mut pin_map: HashMap<_, EdgeSet> = HashMap::new();

    let mut insert = |k: String, v: Endpoint, as_input: bool| {
        match pin_map.entry(k) {
            Entry::Occupied(mut e) => {
                e.get_mut().add(v, as_input)
                // Ok(())
            }
            Entry::Vacant(e) => {
                e.insert(EdgeSet::new_with(v, as_input)?);
                Ok(())
            }
        }
    };

    let input_interface = dependents[conn_names.len()].interface();
    let output_interface = dependents[conn_names.len() + 1].interface();

    for (index, parser::Connection { inputs, .. }) in conn_names.iter().enumerate() {
        let interface = dependents[index].interface();
        let res = inputs.iter().try_for_each::<_, Result<(), ()>>(|argument| {
            let Argument {
                internal,
                internal_bus,
                external,
                external_bus,
            } = argument;
            match external {
                Symbol::Name(external) => {
                    let (external, raw) = {
                        let r = **external;
                        let external = if let Some(bus) = external_bus {
                            format!("{external}.{}.{}", bus.start, bus.end)
                        } else {
                            external.to_string()
                        };
                        (external, r)
                    };

                    if let Ok(bus) = input_interface.real_range(&raw, external_bus.clone()) {
                        println!("Inserting input connection on {}", conn_names.len());
                        insert(
                            external.to_string(),
                            Endpoint {
                                index: conn_names.len(),
                                range: bus,
                            },
                            true, // it's an output of the input bus
                        );
                    } else if let Ok(bus) = output_interface.real_range(&raw, external_bus.clone())
                    {
                        println!("Inserting output connection on {}", conn_names.len() + 1);
                        insert(
                            external.to_string(),
                            Endpoint {
                                index: conn_names.len() + 1,
                                range: bus,
                            },
                            false, // it's an input of the output bus
                        );
                    }

                    println!(
                        "Pin {internal} is {}",
                        if interface.is_input(&internal) {
                            "input"
                        } else {
                            "output"
                        }
                    );
                    insert(
                        external.to_string(),
                        Endpoint {
                            index,
                            range: interface.real_range(internal, internal_bus.clone())?,
                        },
                        !interface.is_input(&internal),
                    )?;
                }
                Symbol::Value(_) => {
                    todo!()
                }
                Symbol::Number(_) => {
                    todo!()
                }
            };

            Ok(())
        });

        if let Err(x) = res {
            println!("error: {x:?}");
        }
    }

    pin_map
}
