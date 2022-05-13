use bitvec::prelude::*;

mod native;
mod builtin;
mod builder;
mod error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Id(u16);

pub trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, args: &BitSlice) -> BitVec;
    fn boxed_clone(&self) -> Box<dyn Chip>;
}
