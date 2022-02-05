mod build;
mod vchip;

use petgraph::Graph;
use crate::model::Chip;
use super::parser::interface::Interface;

pub use build::*;
pub use vchip::*;

pub enum ConnEdge {
    Combinatorial {
        buf: Vec<bool>,
    },
    Sequential {
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

pub struct NativeChip {
    conn_graph: Graph<Box<dyn Chip>, ConnEdge>,
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