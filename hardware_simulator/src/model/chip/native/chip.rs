use crate::model::chip::native::conn_edge::ConnEdge;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use itertools::Itertools;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::prelude::*;
use petgraph::{Direction, Graph};

#[derive(Clone, Debug)]
pub struct NativeChip {
    pub conn_graph: Graph<Chip, ConnEdge>,
    pub interface: Interface,
    pub clocked_chips: Vec<NodeIndex>,
    pub clocked_edges: Vec<EdgeIndex>,
}

impl NativeChip {
    pub fn synthesize_input(&self, ix: NodeIndex) -> Vec<bool> {
        // get the input size of the chip and create a buffer
        let size = self.conn_graph[ix].interface().size_in();
        let mut buf = vec![false; size];

        for conn_edge in self.conn_graph.edges_directed(ix, Direction::Incoming) {
            let (conn_buf, range) = conn_edge.weight().get_with_range();
            assert_eq!(conn_buf.len(), range.size() as usize); // maybe move this assertion somewhere else?

            buf[range.as_range()].copy_from_slice(conn_buf);
        }

        buf
    }

    pub fn send_output(&mut self, ix: NodeIndex, data: &[bool]) -> Vec<NodeIndex> {
        self.conn_graph
            .edges_directed(ix, Direction::Outgoing)
            .map(|conn_edge| {
                let (_, range) = conn_edge.weight().get_with_range();
                let ex = conn_edge.id();
                (range, ex)
            })
            .collect_vec()
            .into_iter()
            .map(|(range, ex)| {
                self.conn_graph[ex].set(&data[range.as_range()]);
                self.conn_graph.edge_endpoints(ex).unwrap().1
            })
            .collect_vec()
    }

    fn step_through(&mut self, ix: NodeIndex) -> Vec<NodeIndex> {
        let input = self.synthesize_input(ix);
        let output = self.conn_graph[ix].eval(&input);
        self.send_output(ix, &output)
    }
}

impl ChipObject for NativeChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn is_clocked(&self) -> bool {
        self.clocked_chips.len() > 0 || self.clocked_edges.len() > 0
    }

    fn clock(&mut self) {
        for node in &self.clocked_chips {
            self.conn_graph[node.clone()].clock();
        }
        for edge in &self.clocked_edges {
            self.conn_graph[edge.clone()].clock();
        }
        // self.eval(); //TODO: Need to propagate changes after edges change
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(self.clone())
    }
}
