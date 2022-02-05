use crate::bus_range::BusRange;
use crate::model::parser::{self, Argument, Interface, Symbol};
use crate::model::Chip;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Endpoint {
    index: usize,
    range: BusRange,
}

pub fn edges_from_connections(
    conn_names: &Vec<parser::Connection>,
    dependents: &Vec<Box<dyn Chip>>,
    input_chip: &HashMap<String, BusRange>,
    output_chip: &HashMap<String, BusRange>,
) -> HashMap<String, Vec<Endpoint>> {
    let mut pin_map: HashMap<_, Vec<_>> = HashMap::new();

    let mut insert = |k: String, v: Endpoint| match pin_map.entry(k) {
        Entry::Occupied(mut e) => {
            e.get_mut().push(v);
        }
        Entry::Vacant(e) => {
            e.insert(vec![v]);
        }
    };

    for (index, parser::Connection { inputs, .. }) in conn_names.iter().enumerate() {
        let interface = dependents[index].interface();
        let _ = inputs.iter().try_for_each::<_, Result<(), ()>>(|argument| {
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

                    if input_chip.contains_key(raw) {
                        insert(
                            external.to_string(),
                            Endpoint {
                                index: conn_names.len(),
                                range: dependents[conn_names.len()]
                                    .interface()
                                    .real_range(&raw, external_bus.clone())?,
                            },
                        )
                    } else if output_chip.contains_key(raw) {
                        insert(
                            external.to_string(),
                            Endpoint {
                                index: conn_names.len() + 1,
                                range: dependents[conn_names.len() + 1]
                                    .interface()
                                    .real_range(&raw, external_bus.clone())?,
                            },
                        )
                    }

                    insert(
                        external.to_string(),
                        Endpoint {
                            index,
                            range: interface.real_range(internal, internal_bus.clone())?,
                        },
                    )
                }
                Symbol::Value(v) => {
                    todo!()
                }
                Symbol::Number(n) => {
                    todo!()
                }
            };

            Ok(())
        });
    }

    pin_map
}
