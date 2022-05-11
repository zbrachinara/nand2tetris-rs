use super::{Chip, Id};
use crate::channel_range::ChannelRange;
use std::collections::HashMap;

struct Router {
    map: Vec<(ChannelRange, (Id, ChannelRange))>,
}

struct Request<'router> {
    id: Id,
    data: &'router [bool],
    range: ChannelRange,
}

struct Barrier {
    in_buffer: Vec<bool>,
    out_buffer: Vec<bool>,
    clock_mask: Vec<bool>,
    chip: Box<dyn Chip>,
    router: Router,
}

struct NativeChip {
    registry: HashMap<Id, Barrier>,
    in_router: Router,
    out_chip: Id,
}
