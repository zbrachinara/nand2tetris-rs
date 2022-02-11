pub mod build;

use petgraph::Graph;
use crate::bus_range::BusRange;
use crate::model::chip::{BuiltinChip, Chip};
use crate::model::parser::Interface;

#[derive(Clone)]
pub enum ConnEdge {
    Combinatorial {
        in_range: BusRange,
        out_range: BusRange,
        buf: Vec<bool>,
    },
    Sequential {
        in_range: BusRange,
        out_range: BusRange,
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

#[derive(Clone)]
pub struct NativeChip {
    pub conn_graph: Graph<Chip, ConnEdge>,
    pub interface: Interface,
}

impl BuiltinChip for NativeChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        todo!()
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }

    fn chip_clone(&self) -> Box<dyn BuiltinChip> {
        Box::new(Self {
            conn_graph: self.conn_graph.clone(),
            interface: self.interface.clone(),
        })
    }
}