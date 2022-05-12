use bitvec::prelude::*;

use super::{Chip, Id};
use crate::channel_range::ChannelRange;
use std::collections::{HashMap, VecDeque};

struct Router {
    map: Vec<(ChannelRange, (Id, ChannelRange))>,
}

/// Represents a request to modify the chip represented by the id with the
/// inside data
///
/// Inside the targeted chip, only the data specified by the range will be
/// modified.
#[derive(Debug)]
struct Request {
    id: Id,
    data: BitVec,
    range: ChannelRange,
}

/// Barrier representing how the chip interacts inside a native chip
///
/// clock_mask is set true for bits which are not clocked. When a clock cycle
/// occurs, all bits pass from in_buffer to intermediate. But when a non-clock
/// eval occurs, only those marked as true in clock_mask pass from in_buffer to
/// intermediate.
struct Barrier {
    in_buffer: BitVec,
    intermediate: BitVec,
    clock_mask: BitVec,
    out_buffer: BitVec,
    chip: Box<dyn Chip>,
    router: Router,
}

pub struct NativeChip {
    registry: HashMap<Id, Barrier>,
    in_router: Router,
    out_chip: Id,
    out_buffer: BitVec,
}

impl Router {
    fn gen_requests(&self, data: &BitSlice) -> impl Iterator<Item = Request> + '_ {
        let copy = data.to_bitvec();
        self.map
            .iter()
            .map(move |(in_range, (id, out_range))| Request {
                id: id.clone(),
                data: copy[in_range.as_range()].to_bitvec(),
                range: out_range.clone(),
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
}

impl Chip for NativeChip {
    fn clock(&mut self) {
        todo!();
    }
    fn eval(&mut self, args: &BitSlice) -> BitVec {
        let mut request_queue: VecDeque<_> = self.in_router.gen_requests(args).collect();
        while let Some(req) = request_queue.pop_front() {
            println!("With request queue: {request_queue:?} and request: {req:?}");
            if req.id == self.out_chip {
                self.out_buffer[req.range.as_range()].copy_from_bitslice(req.data.as_bitslice());
            } else {
                let chip = self.registry.get_mut(&req.id).unwrap();
                chip.accept(&req);
                request_queue.extend(chip.eval())
            }
        }
        self.out_buffer.clone()
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
                    chip: crate::model::chip_v2::builtin::nand().0,
                    router: Router {
                        map: vec![((0..=0).into(), (Id(0), (0..=0).into()))],
                    },
                },
            )]
            .into_iter()
            .collect(),
            in_router: Router {
                map: vec![
                    ((0..=0).into(), (Id(1), (0..=0).into())),
                    ((0..=0).into(), (Id(1), (1..=1).into())),
                ],
            },
            out_chip: Id(0),
            out_buffer: bitvec![0],
        }
    }

    #[test]
    fn not() {
        let mut not = manual_not_chip();
        assert_eq!(not.eval(bits!(0)), bits!(1));
        assert_eq!(not.eval(bits!(1)), bits!(0));
    }
}
