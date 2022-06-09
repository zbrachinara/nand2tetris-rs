use bitvec::prelude::*;

use super::{Chip, Id};
use crate::channel_range::ChannelRange;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub(super) struct Router {
    pub map: Vec<(ChannelRange, Hook)>,
}

/// Represents a request to modify the chip represented by the id with the
/// inside data
///
/// Inside the targeted chip, only the data specified by the range will be
/// modified.
#[derive(Clone)]
pub(super) struct Request {
    id: Id,
    data: BitVec,
    range: ChannelRange,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("id", &self.id)
            .field("data", &format!("{}", self.data))
            .field("range", &self.range)
            .finish()
    }
}

/// Barrier representing how the chip interacts inside a native chip
///
/// clock_mask is set true for bits which are not clocked. When a clock cycle
/// occurs, all bits pass from in_buffer to intermediate. But when a non-clock
/// eval occurs, only those marked as true in clock_mask pass from in_buffer to
/// intermediate.
pub(super) struct Barrier {
    pub in_buffer: BitVec,
    pub intermediate: BitVec,
    pub clock_mask: BitVec,
    pub out_buffer: BitVec,
    pub chip: Box<dyn Chip>,
    pub router: Router,
}

#[derive(Clone)]
pub struct NativeChip {
    pub(super) registry: HashMap<Id, Barrier>,
    pub(super) in_router: Router,
    pub(super) out_chip: Id,
    pub(super) out_buffer: BitVec,
    pub(super) request_queue: VecDeque<Request>,
}

#[derive(Debug, Clone)]
pub(super) struct Hook {
    pub id: Id,
    pub range: ChannelRange,
}

impl Clone for Barrier {
    fn clone(&self) -> Self {
        Self {
            in_buffer: self.in_buffer.clone(),
            intermediate: self.intermediate.clone(),
            clock_mask: self.clock_mask.clone(),
            out_buffer: self.out_buffer.clone(),
            chip: self.chip.boxed_clone(),
            router: self.router.clone(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    pub fn add_hook(&mut self, range: ChannelRange, hook: Hook) {
        self.map.push((range, hook));
    }

    fn gen_requests(&self, data: &BitSlice) -> impl Iterator<Item = Request> + '_ {
        let copy = data.to_bitvec();
        self.map
            .iter()
            .map(move |(in_range, Hook { id, range })| Request {
                id: id.clone(),
                data: copy[in_range.as_range()].to_bitvec(),
                range: range.clone(),
            })
    }
}

impl Barrier {
    fn switch_buffers_eval(&mut self) {
        self.in_buffer
            .iter()
            .zip(self.clock_mask.iter())
            .zip(self.intermediate.iter_mut())
            .for_each(|((in_bit, mask_bit), mut out_bit)| {
                out_bit.set(*mask_bit && *in_bit || !mask_bit && *out_bit)
            });
    }
    fn switch_buffers_clock(&mut self) {
        self.intermediate
            .copy_from_bitslice(self.in_buffer.as_bitslice());
    }
    fn accept(&mut self, req: &Request) {
        self.in_buffer[req.range.as_range()].copy_from_bitslice(req.data.as_bitslice())
    }
    fn eval(&mut self) -> impl Iterator<Item = Request> + '_ {
        self.switch_buffers_eval();
        self.out_buffer = self.chip.eval(self.intermediate.as_bitslice());
        self.router.gen_requests(self.out_buffer.as_bitslice())
    }
    fn clock(&mut self) -> Option<impl Iterator<Item = Request> + '_> {
        (!self.clock_mask.all()).then(|| {
            self.switch_buffers_clock();
            self.out_buffer = self.chip.clock(self.intermediate.as_bitslice());
            self.router.gen_requests(self.out_buffer.as_bitslice())
        })
    }
}

impl Chip for NativeChip {
    fn clock(&mut self, args: &BitSlice) -> BitVec {
        self.request_queue.extend(self.in_router.gen_requests(args));
        while let Some(req) = self.request_queue.pop_front() {
            println!(
                "With request queue: {:?} and request: {req:?}",
                self.request_queue
            );
            if req.id == self.out_chip {
                self.out_buffer[req.range.as_range()].copy_from_bitslice(req.data.as_bitslice());
            } else {
                let chip = self.registry.get_mut(&req.id).unwrap();
                chip.accept(&req);
                chip.clock()
                    .map(|requests| self.request_queue.extend(requests));
            }
        }
        self.out_buffer.clone()
    }

    fn eval(&mut self, args: &BitSlice) -> BitVec {
        self.request_queue.extend(self.in_router.gen_requests(args));
        while let Some(req) = self.request_queue.pop_front() {
            println!(
                "With request queue: {:?} and request: {req:?}",
                self.request_queue
            );
            if req.id == self.out_chip {
                self.out_buffer[req.range.as_range()].copy_from_bitslice(req.data.as_bitslice());
            } else {
                let chip = self.registry.get_mut(&req.id).unwrap();
                chip.accept(&req);
                self.request_queue.extend(chip.eval())
            }
        }
        self.out_buffer.clone()
    }

    fn boxed_clone(&self) -> Box<dyn Chip> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn manual_not_chip() -> NativeChip {
        NativeChip {
            registry: [(
                Id(1),
                Barrier {
                    in_buffer: bitvec!(0, 0),
                    intermediate: bitvec!(0, 0),
                    clock_mask: bitvec!(1, 1),
                    out_buffer: bitvec!(0, 0),
                    chip: crate::model::chip::builtin::nand().chip,
                    router: Router {
                        map: vec![(
                            (0..=0).into(),
                            Hook {
                                id: Id(0),
                                range: (0..=0).into(),
                            },
                        )],
                    },
                },
            )]
            .into_iter()
            .collect(),
            in_router: Router {
                map: vec![
                    (
                        (0..=0).into(),
                        Hook {
                            id: Id(1),
                            range: (0..=0).into(),
                        },
                    ),
                    (
                        (0..=0).into(),
                        Hook {
                            id: Id(1),
                            range: (1..=1).into(),
                        },
                    ),
                ],
            },
            out_chip: Id(0),
            out_buffer: bitvec![0],
            request_queue: VecDeque::new(),
        }
    }

    #[test]
    fn not() {
        let mut not = manual_not_chip();
        assert_eq!(not.eval(bits!(0)), bits!(1));
        assert_eq!(not.eval(bits!(1)), bits!(0));
    }
}
