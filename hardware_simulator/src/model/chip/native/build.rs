use crate::bus_range::BusRange;
use crate::model::chip::build_ctx::FileContext;
use crate::model::chip::native::{ConnEdge, NativeChip};
use crate::model::chip::vchip::VirtualBus;
use crate::model::chip::Chip;
use crate::model::parser::{Argument, Connection, Interface, Symbol};
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::borrow::Cow;
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

    fn iter(&self) -> Result<impl Iterator<Item = (&Endpoint, &Endpoint)>, ()> {
        self.input
            .as_ref()
            .map(|i| self.outputs.iter().map(move |x| (i, x)))
            .ok_or(())
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

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub index: NodeIndex,
    pub range: BusRange,
    pub com_or_seq: ClockBehavior,
}

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
