use crate::bus_range::BusRange;
use crate::model::parser::{Argument, Connection as ConnRepr, Interface, Symbol};
use crate::model::Chip;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EdgeRepr {
    index: usize,
    range: BusRange,
}

pub fn edges_from_connections(
    conn_names: &Vec<ConnRepr>,
    dependents: &Vec<Box<dyn Chip>>,
    input_chip: &HashMap<String, BusRange>,
    output_chip: &HashMap<String, BusRange>,
) -> HashMap<String, Vec<EdgeRepr>> {
    let mut pin_map: HashMap<_, Vec<_>> = HashMap::new();

    let mut insert = |k: String, v: EdgeRepr| match pin_map.entry(k) {
        Entry::Occupied(mut e) => {
            e.get_mut().push(v);
        }
        Entry::Vacant(e) => {
            e.insert(vec![v]);
        }
    };

    conn_names
        .iter()
        .enumerate()
        .for_each(|(index, ConnRepr { inputs, .. })| {
            let interface = dependents[index].interface();
            let _ = inputs.iter().try_for_each::<_, Result<(), ()>>(
                // TODO: Cleanup, maybe extract into another method
                |Argument {
                     internal,
                     internal_bus,
                     external,
                     external_bus,
                 }| {
                    match external {
                        Symbol::Name(external) => {

                            let mut external = external.to_string();

                            let raw = {
                                let r = external.clone();
                                external = if let Some(bus) = external_bus {
                                    format!("{external}.{}.{}", bus.start, bus.end)
                                } else {
                                    external
                                };
                                r
                            };

                            if input_chip.contains_key(&raw) {
                                insert(
                                    external.to_string(),
                                    EdgeRepr {
                                        index: conn_names.len(),
                                        range: dependents[conn_names.len()].interface().real_range(&raw, external_bus.clone())?
                                    }
                                )
                            } else if output_chip.contains_key(&raw) {
                                insert(
                                    external.to_string(),
                                    EdgeRepr {
                                        index: conn_names.len() + 1,
                                        range: dependents[conn_names.len() + 1].interface().real_range(&raw, external_bus.clone())?
                                    }
                                )
                            }

                            insert(
                                external.to_string(),
                                EdgeRepr {
                                    index,
                                    range: interface.real_range(internal, internal_bus.clone())?,
                                },
                            )
                        },
                        Symbol::Value(v) => {
                            todo!()
                        }
                        Symbol::Number(n) => {
                            todo!()
                        }
                    };

                    Ok(())
                },
            );
        });

    pin_map
}
