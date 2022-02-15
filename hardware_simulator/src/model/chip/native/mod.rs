pub mod build;
mod edge_set;

use crate::bus_range::BusRange;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use petgraph::Graph;
use std::fmt::{Display, Formatter};
use petgraph::graph::{NodeIndex, EdgeIndex};

#[derive(Clone, Debug)]
pub enum ConnEdge {
    Combinatorial {
        name: String,
        in_range: BusRange,
        out_range: BusRange,
        buf: Vec<bool>,
    },
    Sequential {
        name: String,
        in_range: BusRange,
        out_range: BusRange,
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

impl Display for ConnEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Combinatorial { name, .. } => name,
                Self::Sequential { name, .. } => name,
            }
        )
    }
}

impl ConnEdge {
    fn new_com(name: String, in_range: BusRange, out_range: BusRange) -> Self {
        let size = in_range.size() as usize;
        Self::Combinatorial {
            name,
            in_range,
            out_range,
            buf: Vec::with_capacity(size),
        }
    }
    fn new_seq(name: String, in_range: BusRange, out_range: BusRange) -> Self {
        let size = in_range.size() as usize;
        Self::Sequential {
            name,
            in_range,
            out_range,
            waiting: Vec::with_capacity(size),
            buf: Vec::with_capacity(size),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NativeChip {
    pub conn_graph: Graph<Chip, ConnEdge>,
    pub interface: Interface,
    pub clocked_chips: Vec<NodeIndex>,
    pub clocked_edges: Vec<EdgeIndex>,
}

impl ChipObject for NativeChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn is_clocked(&self) -> bool {
        self.clocked_chips.len() > 0 || self.clocked_edges.len() > 0
    }

    fn clock(&mut self) {
        todo!()
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(self.clone())
    }
}
