
use petgraph::Graph;
use crate::model::Chip;

enum Conn {
    Combinatorial {
        buf: Vec<bool>,
    },
    Sequential {
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

struct NativeChip {
    conn_graph: Graph<Box<dyn Chip>, Conn>,
}