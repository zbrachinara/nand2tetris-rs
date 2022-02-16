use crate::model::chip::native::conn_edge::ConnEdge;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;

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
