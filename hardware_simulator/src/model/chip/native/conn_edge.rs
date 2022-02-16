use std::fmt::{Display, Formatter};
use crate::bus_range::BusRange;

#[derive(Clone, Debug)]
pub enum ConnEdge {
    Combinatorial {
        name: String,
        in_range: BusRange,
        out_range: BusRange,
        buf: Vec<bool>,
    },
    Sequential {
        name: String,
        in_range: BusRange,
        out_range: BusRange,
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

impl Display for ConnEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Combinatorial { name, .. } => name,
                Self::Sequential { name, .. } => name,
            }
        )
    }
}

impl ConnEdge {
    pub fn new_com(name: String, in_range: BusRange, out_range: BusRange) -> Self {
        let size = in_range.size() as usize;
        Self::Combinatorial {
            name,
            in_range,
            out_range,
            buf: Vec::with_capacity(size),
        }
    }
    pub fn new_seq(name: String, in_range: BusRange, out_range: BusRange) -> Self {
        let size = in_range.size() as usize;
        Self::Sequential {
            name,
            in_range,
            out_range,
            waiting: Vec::with_capacity(size),
            buf: Vec::with_capacity(size),
        }
    }
}
