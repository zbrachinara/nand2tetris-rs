use super::{Chip, Id};
use crate::channel_range::ChannelRange;
use std::{collections::HashMap, ops::RangeInclusive};

struct Router {
    map: Vec<(ChannelRange, (Id, ChannelRange))>,
}

struct Request<'data> {
    id: Id,
    data: &'data [bool],
    range: ChannelRange,
}

struct Barrier {
    in_buffer: Vec<bool>,
    out_buffer: Vec<bool>,
    clock_mask: Vec<bool>,
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
        data: &'data [bool],
    ) -> impl Iterator<Item = Request<'data>> + 'out {
        self.map.iter().map(|(in_range, (id, out_range))| Request {
            id: id.clone(),
            data: &data[RangeInclusive::from(in_range.clone())],
            range: *out_range,
        })
    }
}

impl Chip for NativeChip {
    fn clock(&mut self) {
        todo!();
    }
    fn eval(&mut self, args: &[bool]) {
        todo!();
    }
}
