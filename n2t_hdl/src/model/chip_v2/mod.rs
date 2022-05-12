use bitvec::prelude::*;

mod native;
mod builtin;

#[derive(Clone)]
struct Id(u16);

trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, args: &BitSlice) -> BitVec;
}
