
use petgraph::Graph;
use crate::model::Chip;

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