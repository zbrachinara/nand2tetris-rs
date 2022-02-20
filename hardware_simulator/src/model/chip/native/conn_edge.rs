use crate::channel_range::ChannelRange;
use std::fmt::{Display, Formatter};
use bitvec::prelude::{BitSlice, BitVec};

#[derive(Clone, Debug)]
pub enum ConnEdge {
    Combinatorial {
        name: String,
        in_range: ChannelRange,
        out_range: ChannelRange,
        buf: BitVec,
    },
    Sequential {
        name: String,
        in_range: ChannelRange,
        out_range: ChannelRange,
        waiting: BitVec,
        buf: BitVec,
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
    pub fn new_com(name: String, in_range: ChannelRange, out_range: ChannelRange) -> Self {
        let size = in_range.size() as usize;
        Self::Combinatorial {
            name,
            in_range,
            out_range,
            buf: BitVec::repeat(false, size),
        }
    }
    pub fn new_seq(name: String, in_range: ChannelRange, out_range: ChannelRange) -> Self {
        let size = in_range.size() as usize;
        Self::Sequential {
            name,
            in_range,
            out_range,
            waiting: BitVec::repeat(false, size),
            buf: BitVec::repeat(false, size),
        }
    }

    pub fn clock(&mut self) {
        if let Self::Sequential { waiting, buf, .. } = self {
            buf.copy_from_bitslice(waiting);
        }
    }

    pub fn get_with_range_out(&self) -> (&BitSlice, ChannelRange) {
        match self {
            ConnEdge::Combinatorial { buf, out_range, .. } => (buf.as_ref(), out_range.clone()),
            ConnEdge::Sequential { buf, out_range, .. } => (buf.as_ref(), out_range.clone()),
        }
    }

    pub fn get_range_in(&self) -> &ChannelRange {
        match self {
            Self::Combinatorial { in_range, ..} => in_range,
            Self::Sequential { in_range, ..} => in_range,
        }
    }

    pub fn set(&mut self, new_buf: &BitSlice) {
        match self {
            ConnEdge::Combinatorial { buf, .. } => buf.copy_from_bitslice(new_buf),
            ConnEdge::Sequential{ waiting, .. } => waiting.copy_from_bitslice(new_buf),

        }
    }
}
