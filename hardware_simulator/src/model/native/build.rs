use std::collections::HashMap;
use crate::bus_range::BusRange;
use crate::model::Chip;
use crate::parser::{Argument, Connection, Symbol};

pub fn connections_by_pin(
    connections: &Vec<Connection>,
    dependents: &Vec<Box<dyn Chip>>,
) -> HashMap<String, Vec<(usize, BusRange)>> {
    let mut pin_map = HashMap::new();

    let mut insert = |k: String, v: (usize, BusRange)| {
        pin_map
            .entry(k)
            .and_modify(|e: &mut Vec<_>| e.push(v.clone()))
            .or_insert(vec![v]);
    };

    connections
        .iter()
        .enumerate()
        .for_each(|(index, Connection { inputs, .. })| {
            let interface = dependents[index].interface();
            let _ = inputs.iter().try_for_each::<_, Result<(), ()>>(
                |Argument {
                     internal,
                     internal_bus,
                     external,
                     ..
                 }| {
                    // TODO: Handle output pin indexing
                    if let Symbol::Name(external) = external {
                        insert(
                            external.to_string(),
                            (index, interface.real_range(internal, internal_bus.clone())?),
                        )
                    }

                    Ok(())
                },
            );
        });

    pin_map
}
