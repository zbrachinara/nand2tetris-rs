use crate::bus_range::BusRange;
use crate::model::chip::build_ctx::FileContext;
use crate::model::chip::native::ConnEdge;
use crate::model::chip::vchip::VirtualBus;
use crate::model::chip::Chip;
use crate::model::parser::{Argument, Connection, Interface, Symbol};
use itertools::Itertools;
use std::borrow::Cow;
// use petgraph::data::{Element, FromElements};
use crate::Span;
use derive_more::{Deref, DerefMut};
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

#[derive(Debug, Deref, DerefMut)]
struct EdgeSetMap(HashMap<String, EdgeSet>);

impl EdgeSetMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, k: String, v: Endpoint, input: bool) -> Result<(), ()> {
        match self.entry(k) {
            Entry::Occupied(mut e) => {
                e.get_mut().add(v, input)?;
            }
            Entry::Vacant(e) => {
                e.insert(EdgeSet::new_with(v, input)?);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Endpoint {
    pub index: NodeIndex,
    pub range: BusRange,
    pub com_or_seq: ClockBehavior,
}

#[derive(Debug)]
pub enum ClockBehavior {
    Combinatorial,
    Sequential,
}

struct Dependency<'a> {
    index: NodeIndex,
    interface: Interface,
    connections: Vec<Argument<'a>>,
}

pub fn native_chip(
    ctx: &FileContext,
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
    let dependents = connections
        .into_iter()
        .filter_map(|Connection { chip_name, inputs }| {
            ctx.resolve_chip_maybe_builtin(*chip_name).map(|chip| {
                let interface = chip.interface();
                let index = conn_graph.add_node(chip);
                Dependency {
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
    let (input_index, output_index) = (
        conn_graph.add_node(Box::new(input)),
        conn_graph.add_node(Box::new(output)),
    );

    let mut edge_sets = EdgeSetMap::new();
    for Dependency {
        index,
        interface,
        connections,
    } in dependents
    {
        for argument in connections {
            match argument.external {
                Symbol::Name(pin_name) => {
                    let Argument {
                        internal,
                        internal_bus,
                        external_bus,
                        ..
                    } = argument;
                    let pin_name = *pin_name;
                    let canonical_pin_name = if let Some(ref external_bus) = external_bus {
                        Cow::Owned(format!(
                            "{pin_name}.{}.{}",
                            external_bus.start, external_bus.end
                        ))
                    } else {
                        Cow::Borrowed(pin_name)
                    };

                    // automatic hooking to input/output pins
                    if let Ok(range) = input_interface.real_range(pin_name, external_bus.as_ref()) {
                        edge_sets.insert(
                            canonical_pin_name.to_string(),
                            Endpoint {
                                range,
                                index: input_index,
                                com_or_seq: ClockBehavior::Combinatorial,
                            },
                            true,
                        );
                    } else if let Ok(range) =
                        output_interface.real_range(pin_name, external_bus.as_ref())
                    {
                        edge_sets.insert(
                            canonical_pin_name.to_string(),
                            Endpoint {
                                range,
                                index: output_index,
                                com_or_seq: ClockBehavior::Combinatorial,
                            },
                            false,
                        );
                    } else {
                        if matches!(external_bus, Some(_)) {
                            return Err(());
                        }
                    }

                    edge_sets.insert(
                        (*pin_name).to_string(),
                        Endpoint {
                            index,
                            range: interface.real_range(*internal, internal_bus.as_ref())?,
                            com_or_seq: ClockBehavior::Combinatorial, // TODO: Get this from the interface
                        },
                        !interface.is_input(*internal),
                    )?;
                }
                Symbol::Value(_) => todo!(),
                Symbol::Number(_) => panic!("Numbers are not supported by this hack hdl version"),
            }
        }
    }

    println!("{edge_sets:#?}");

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
