use std::collections::hash_map::Entry;
use crate::bus_range::BusRange;
use crate::model::parser::{Argument, Connection as ConnRepr, Symbol};
use crate::model::Chip;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Connection {
    index: usize,
    range: BusRange,
}

pub fn connections_by_pin(
    connections: &Vec<ConnRepr>,
    dependents: &Vec<Box<dyn Chip>>,
) -> HashMap<String, Vec<Connection>> {
    let mut pin_map: HashMap<_, Vec<_>> = HashMap::new();

    let mut insert = |k: String, v: Connection| {
        match pin_map.entry(k) {
            Entry::Occupied(mut e) => {e.get_mut().push(v);}
            Entry::Vacant(e) => {e.insert(vec![v]);}
        }
    };

    connections
        .iter()
        .enumerate()
        .for_each(|(index, ConnRepr { inputs, .. })| {
            let interface = dependents[index].interface();
            let _ = inputs.iter().try_for_each::<_, Result<(), ()>>(
                |Argument {
                     internal,
                     internal_bus,
                     external,
                     ..
                 }| {
                    // TODO: Handle output pin indexing
                    match external {
                        Symbol::Name(external) => insert(
                            external.to_string(),
                            Connection {
                                index,
                                range: interface.real_range(internal, internal_bus.clone())?,
                            },
                        ),
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
