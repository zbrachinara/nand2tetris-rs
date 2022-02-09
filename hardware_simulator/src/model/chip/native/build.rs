use crate::bus_range::BusRange;
use crate::model::chip::build_ctx::Context;
use crate::model::chip::native::ConnEdge;
use crate::model::chip::vchip::VirtualBus;
use crate::model::chip::Chip;
use crate::model::parser::{self, Argument, Connection, Interface, Symbol};
use itertools::Itertools;
use petgraph::data::{Element, FromElements};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
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
    pub com_or_seq: ClockBehavior,
}

#[derive(Debug)]
pub enum ClockBehavior {
    Combinatorial,
    Sequential,
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
                                com_or_seq: ClockBehavior::Combinatorial,
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
                                com_or_seq: ClockBehavior::Combinatorial,
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
                            com_or_seq: ClockBehavior::Combinatorial,
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

struct ConnDesc<'a> {
    index: NodeIndex<u32>,
    interface: Interface,
    connections: Vec<Argument<'a>>,
}

pub fn native_chip(
    ctx: &Context,
    top_interface: Interface,
    connections: Vec<Connection>,
) -> Result<Box<dyn Chip>, ()> {
    let Interface {
        com_in, com_out, ..
    } = top_interface;
    let (input, output) = (VirtualBus::new_in(com_in), VirtualBus::new_out(com_out));
    println!("External interface: \n{input:?}\n{output:?}");

    let mut conn_graph = Graph::<_, ConnEdge>::new();

    // instantiate all chips this chip depends on
    let mut dependents = connections
        .into_iter()
        .filter_map(|Connection { chip_name, inputs }| {
            ctx.resolve_chip_maybe_builtin(*chip_name).map(|chip| {
                let interface = chip.interface();
                let index = conn_graph.add_node(chip);
                ConnDesc {
                    index,
                    interface,
                    connections: inputs,
                }
            })
        })
        .collect_vec();

    // insert the input and output
    let input_interface = input.interface();
    let output_interface = output.interface();
    let (in_index, out_index) = (
        conn_graph.add_node(Box::new(input)),
        conn_graph.add_node(Box::new(output)),
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
