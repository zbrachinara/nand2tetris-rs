pub mod build;
pub mod vchip;

use petgraph::Graph;
use crate::bus_range::BusRange;
use crate::model::Chip;
use super::parser::interface::Interface;

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
}

impl Chip for NativeChip {
    fn interface(&self) -> Interface {
        todo!()
    }

    fn clock(&mut self) {
        todo!()
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }
}