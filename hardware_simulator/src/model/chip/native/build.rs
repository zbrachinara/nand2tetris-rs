use crate::bus_range::BusRange;
use crate::model::parser::{self, Argument, Chip as ChipRepr, Connection, Interface, Symbol};
use crate::model::chip::Chip;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::once;
use petgraph::Graph;
use petgraph::data::{Element, FromElements};
use petgraph::graph::NodeIndex;
use itertools::Itertools;
use crate::model::chip::build_ctx::Context;
use crate::model::chip::native::ConnEdge;
use crate::model::chip::native::vchip::VirtualBus;

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

fn edges_from_connections(
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

pub fn native_chip(
    ctx: &Context,
    chip_repr: &ChipRepr,
    connections: &Vec<Connection>,
) -> Result<Box<dyn Chip>, ()> {
    println!("Evaluating {}", chip_repr.name);

    let Interface {
        com_in, com_out, ..
    } = chip_repr.interface();
    let (input, output) = (
        VirtualBus::new_in(com_in.clone()),
        VirtualBus::new_out(com_out.clone()),
    );
    println!("External interface: \n{input:?}\n{output:?}");

    // instantiate all chips this chip depends on
    let mut dependents = connections
        .iter()
        .filter_map(|Connection { chip_name, .. }| ctx.resolve_chip_maybe_builtin(**chip_name))
        .chain(once(Box::new(input) as Box<dyn Chip>))
        .chain(once(Box::new(output) as Box<dyn Chip>))
        .collect_vec();

    let mut graph = Graph::<_, ConnEdge>::from_elements(
        dependents.iter().map(|chip| Element::Node { weight: chip }),
    );

    // get list of all pins and their connections
    // This is done by checking in which `Connection` the name of the pin appears
    // let pins = edges_from_connections(connections, &mut dependents);

    // println!("{pins:#?}");

    // starting from the output pins, build a graph of all connections between chips
    // should work recursively, but also be aware of chips which were already found

    // let mut graph = Graph::<_, ConnEdge>::from_elements(
    //     dependents.into_iter().map(|x| Element::Node { weight: x }),
    // );

    // check for contradictions (one pin with many sources, incompatible channel sizes, etc)
    // while changing edge sets to pairs
    // for (name, edge_set) in pins {
    //     let input = edge_set.input.ok_or(())?;
    //     if edge_set.outputs.len() == 0 {
    //         println!("No output!");
    //         return Err(());
    //     }
    //     for output in edge_set.outputs {
    //         if input.range.size() == output.range.size() {
    //             // TODO: Function to determine whether combinatorial or sequential
    //             graph.add_edge(
    //                 NodeIndex::new(input.index),
    //                 NodeIndex::new(output.index),
    //                 ConnEdge::Combinatorial {
    //                     range: output.range,
    //                     buf: Vec::with_capacity(input.range.size() as usize),
    //                 },
    //             );
    //         } else {
    //             println!("No input!");
    //             return Err(());
    //         }
    //     }
    // }

    // Ok(Box::new(NativeChip {
    //     conn_graph: graph,
    //     interface: chip_repr.interface(),
    // }))
    todo!()
}
