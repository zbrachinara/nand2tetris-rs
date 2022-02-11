use super::edge_set::{EdgeSetMap, Endpoint};
use crate::model::chip::build_ctx::FileContext;
use crate::model::chip::native::{ConnEdge, NativeChip};
use crate::model::chip::vchip::VirtualBus;
use crate::model::chip::Chip;
use crate::model::parser::{Argument, Connection, Interface, Symbol};
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum ClockBehavior {
    Combinatorial,
    Sequential,
}

impl ClockBehavior {
    fn and(&self, rhs: &Self) -> Self {
        if matches!(self, ClockBehavior::Sequential) || matches!(rhs, ClockBehavior::Sequential) {
            ClockBehavior::Sequential
        } else {
            ClockBehavior::Combinatorial
        }
    }
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
) -> Result<NativeChip, ()> {
    let Interface {
        com_in, com_out, ..
    } = top_interface.clone();
    let (input, output) = (VirtualBus::new_in(com_in), VirtualBus::new_out(com_out));

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
    // including the input and output virtual chips
    let (input_index, output_index) = (conn_graph.add_node(input), conn_graph.add_node(output));

    let edge_sets = make_edge_set(input_index, output_index, &mut conn_graph, dependents)?;

    for (_, set) in edge_sets.iter() {
        for (input, output) in set.iter()? {
            (input.range.size() == output.range.size()).then(|| {
                if matches!(
                    input.com_or_seq.and(&output.com_or_seq),
                    ClockBehavior::Sequential
                ) {
                    conn_graph.add_edge(
                        input.index,
                        output.index,
                        ConnEdge::Sequential {
                            in_range: input.range.clone(),
                            out_range: output.range.clone(),
                            waiting: vec![],
                            buf: vec![],
                        },
                    )
                } else {
                    conn_graph.add_edge(
                        input.index,
                        output.index,
                        ConnEdge::Combinatorial {
                            in_range: input.range.clone(),
                            out_range: output.range.clone(),
                            buf: vec![],
                        },
                    )
                }
            });
        }
    }

    Ok(NativeChip {
        conn_graph,
        interface: top_interface,
    })
}

fn make_edge_set(
    input_index: NodeIndex,
    output_index: NodeIndex,
    conn_graph: &mut Graph<Chip, ConnEdge>,
    dependents: Vec<Dependency>,
) -> Result<EdgeSetMap, ()> {
    // insert the input and output
    let input_interface = conn_graph[input_index].interface();
    let output_interface = conn_graph[output_index].interface();

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
                    if !edge_sets.contains_key(&*canonical_pin_name) {
                        if let Ok(range) =
                            input_interface.real_range(pin_name, external_bus.as_ref())
                        {
                            edge_sets.insert(
                                canonical_pin_name.to_string(),
                                Endpoint {
                                    range,
                                    index: input_index,
                                    com_or_seq: ClockBehavior::Combinatorial,
                                },
                                true,
                            )?;
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
                            )?;
                        } else {
                            if matches!(external_bus, Some(_)) {
                                return Err(());
                            }
                        }
                    }

                    edge_sets.insert(
                        canonical_pin_name.to_string(),
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

    Ok(edge_sets)
}
