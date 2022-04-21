use crate::model::chip::native::conn_edge::ConnEdge;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use bitvec::prelude::*;
use itertools::Itertools;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::prelude::*;
use petgraph::{Direction, Graph};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct NativeChip {
    pub conn_graph: Graph<Chip, ConnEdge>,
    pub interface: Interface,
    pub clocked_chips: Vec<NodeIndex>,
    pub clocked_edges: Vec<EdgeIndex>,
    pub input_index: NodeIndex,
    pub output_index: NodeIndex,
}

impl NativeChip {
    pub fn synthesize_input(&self, ix: NodeIndex) -> BitVec {
        // get the input size of the chip and create a buffer
        let size = self.conn_graph[ix].interface().size_in();
        let mut buf = BitVec::repeat(false, size);
        // let mut buf = vec![false; size];

        for conn_edge in self.conn_graph.edges_directed(ix, Direction::Incoming) {
            let (conn_buf, range) = conn_edge.weight().get_with_range_out();
            assert_eq!(conn_buf.len(), range.size()); // maybe move this assertion somewhere else?

            buf[range.as_range()].copy_from_bitslice(conn_buf);
        }

        buf
    }

    pub fn send_output(&mut self, ix: NodeIndex, data: &BitSlice) -> Vec<NodeIndex> {
        self.conn_graph
            .edges_directed(ix, Direction::Outgoing)
            .map(|conn_edge| {
                let range = conn_edge.weight().get_range_in().clone();
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

    fn step_from_input(&mut self, input: &BitSlice) -> Vec<NodeIndex> {
        self.send_output(self.input_index, &input)
    }

    fn eval_with_beginnings(&mut self, ixs: impl IntoIterator<Item = NodeIndex>) -> BitVec {
        let mut set = ixs.into_iter().collect::<HashSet<_>>();
        while let Some(ix) = set.iter().nth(0).cloned() {
            // removal has to occur *before* evaluation because the resultant could have a feedback
            // loop to itself
            set.remove(&ix);
            self.step_through(ix).drain(..).for_each(|new_ix| {
                set.insert(new_ix);
            });
        }
        self.synthesize_input(self.output_index)
    }

    fn eval_from_input(&mut self, input: &BitSlice) -> BitVec {
        let ixs = self.step_from_input(input);
        self.eval_with_beginnings(ixs)
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

        let next_nodes = self
            .clocked_edges
            .iter()
            .cloned()
            .filter_map(|ex| self.conn_graph.edge_endpoints(ex).map(|(_, ix)| ix))
            .chain(self.clocked_chips.iter().cloned())
            .collect_vec();
        self.eval_with_beginnings(next_nodes); //TODO: Test
    }

    fn eval(&mut self, input: &BitSlice) -> BitVec {
        self.eval_from_input(input)
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(self.clone())
    }
}
