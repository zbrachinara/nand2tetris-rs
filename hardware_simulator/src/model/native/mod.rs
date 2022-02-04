mod build;

use petgraph::Graph;
use crate::model::Chip;
use crate::parser::interface::Interface;

pub use build::*;

pub enum Conn {
    Combinatorial {
        buf: Vec<bool>,
    },
    Sequential {
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

pub struct NativeChip {
    conn_graph: Graph<Box<dyn Chip>, Conn>,
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