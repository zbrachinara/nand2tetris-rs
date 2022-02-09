pub mod build;
pub mod vchip;

use petgraph::Graph;
use crate::bus_range::BusRange;
use crate::model::Chip;
use super::parser::interface::Interface;

#[derive(Clone)]
pub enum ConnEdge {
    Combinatorial {
        range: BusRange,
        buf: Vec<bool>,
    },
    Sequential {
        range: BusRange,
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

pub struct NativeChip {
    pub conn_graph: Graph<Box<dyn Chip>, ConnEdge>,
    pub interface: Interface,
}

impl Chip for NativeChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        todo!()
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }

    fn chip_clone(&self) -> Box<dyn Chip> {
        Box::new(Self {
            conn_graph: self.conn_graph.clone(),
            interface: self.interface.clone(),
        })
    }
}