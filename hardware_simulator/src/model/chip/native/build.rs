use super::edge_set::{EdgeSetMap, Endpoint};
use crate::clock_behavior::ClockBehavior;
use crate::model::chip::build_ctx::ChipBuilder;
use crate::model::chip::native::chip::NativeChip;
use crate::model::chip::native::conn_edge::ConnEdge;
use crate::model::chip::vchip::VirtualBus;
use crate::model::chip::Chip;
use crate::model::parser::{Argument, Connection, Interface, Symbol};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::borrow::Cow;

struct Dependency<'a> {
    index: NodeIndex,
    interface: Interface,
    connections: Vec<Argument<'a>>,
}

pub fn native_chip(
    ctx: &mut ChipBuilder,
    top_interface: Interface,
    connections: Vec<Connection>,
) -> Result<NativeChip, ()> {
    let Interface {
        com_in, com_out, ..
    } = top_interface.clone();
    let (input, output) = (VirtualBus::new_in(com_in), VirtualBus::new_out(com_out));

    let mut conn_graph = Graph::<_, ConnEdge>::new();

    // instantiate all chips this chip depends on
    let (dependents, clocked_chips) = {
        let mut dependents = vec![];
        let mut clocked_chips = vec![];
        for Connection { chip_name, inputs } in connections {
            let chip = ctx.resolve_chip(*chip_name).map_err(|_| ())?;
            let clocked = chip.is_clocked();
            let dependency = ctx
                .resolve_chip(*chip_name)
                .map(|chip| {
                    let interface = chip.interface();
                    let index = conn_graph.add_node(chip);
                    Dependency {
                        index,
                        interface,
                        connections: inputs,
                    }
                })
                .map_err(|_| ())?;
            if clocked {
                clocked_chips.push(dependency.index)
            }
            dependents.push(dependency);
        }

        (dependents, clocked_chips)
    };
    // including the input and output virtual chips
    let (input_index, output_index) = (conn_graph.add_node(input), conn_graph.add_node(output));

    let edge_sets = make_edge_set(input_index, output_index, &mut conn_graph, dependents)?;

    let mut clocked_edges = vec![];
    for (name, set) in edge_sets.iter() {
        for (input, output) in set.iter()? {
            (input.range.size() == output.range.size()).then(|| {
                if matches!(
                    input.clocked.and(&output.clocked),
                    ClockBehavior::Sequential
                ) {
                    clocked_edges.push(conn_graph.add_edge(
                        input.index,
                        output.index,
                        ConnEdge::new_seq(name.clone(), input.range.clone(), output.range.clone()),
                    ));
                } else {
                    conn_graph.add_edge(
                        input.index,
                        output.index,
                        ConnEdge::new_com(name.clone(), input.range.clone(), output.range.clone()),
                    );
                }
            });
        }
    }

    Ok(NativeChip {
        conn_graph,
        interface: top_interface,
        clocked_chips,
        clocked_edges,
        input_index,
        output_index,
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
                                    clocked: ClockBehavior::Combinatorial,
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
                                    clocked: ClockBehavior::Combinatorial,
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
                            clocked: interface.clocked(*internal),
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
