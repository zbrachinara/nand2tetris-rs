use bitvec::prelude::*;

mod builder;
mod builtin;
mod error;
mod native;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Id(u16);

impl Id {
    fn next(&mut self) -> Self {
        let res = self.clone();
        self.0 += 1;
        res
    }
}

pub trait Chip {
    fn clock(&mut self, args: &BitSlice) -> BitVec;
    fn eval(&mut self, args: &BitSlice) -> BitVec;
    fn boxed_clone(&self) -> Box<dyn Chip>;
}
