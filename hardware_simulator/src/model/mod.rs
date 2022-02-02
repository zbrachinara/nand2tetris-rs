use crate::parser::Interface;

mod build_ctx;
mod builtin;

pub trait Chip {
    fn interface(&self) -> Interface;

    fn clock(&mut self);
    fn eval(&mut self, _: &[bool]) -> Vec<bool>;
}