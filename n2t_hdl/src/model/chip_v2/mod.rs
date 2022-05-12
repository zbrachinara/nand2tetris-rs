use bitvec::prelude::*;

mod native;
mod builtin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Id(u16);

pub trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, args: &BitSlice) -> BitVec;
}
