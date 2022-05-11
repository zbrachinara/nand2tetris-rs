use bitvec::prelude::*;

use super::{Chip, Id};
use crate::channel_range::ChannelRange;
use std::collections::HashMap;

struct Router {
    map: Vec<(ChannelRange, (Id, ChannelRange))>,
}

/// Represents a request to modify the chip represented by the id with the
/// inside data
///
/// Inside the targeted chip, only the data specified by the range will be
/// modified.
struct Request<'data> {
    id: Id,
    data: &'data BitSlice,
    range: ChannelRange,
}

/// Barrier representing how the chip interacts inside a native chip
///
/// clock_mask is set true for bits which are not clocked. When a clock cycle
/// occurs, all bits pass from in_buffer to out_buffer. But when a non-clock
/// eval occurs, only those marked as true in clock_mask pass from in_buffer to
/// out_buffer.
struct Barrier {
    in_buffer: BitVec,
    out_buffer: BitVec,
    clock_mask: BitVec,
    chip: Box<dyn Chip>,
    router: Router,
}

pub struct NativeChip {
    registry: HashMap<Id, Barrier>,
    in_router: Router,
    out_chip: Id,
}

impl Router {
    fn gen_requests<'router: 'out, 'data: 'out, 'out>(
        &'router self,
        data: &'data BitSlice,
    ) -> impl Iterator<Item = Request<'data>> + 'out {
        self.map.iter().map(|(in_range, (id, out_range))| Request {
            id: id.clone(),
            data: &data[in_range.as_range()],
            range: *out_range,
        })
    }
}

impl Barrier {
    fn accept(&mut self, req: &Request) {
        self.in_buffer[req.range.as_range()].copy_from_bitslice(req.data)
    }
    fn eval(&mut self) -> impl Iterator<Item = Request> {
        // pass the buffer from outside to inside

        std::iter::empty()
    }
}

impl Chip for NativeChip {
    fn clock(&mut self) {
        todo!();
    }
    fn eval(&mut self, args: &BitSlice) -> &BitSlice {
        todo!();
    }
}
