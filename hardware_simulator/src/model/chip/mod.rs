use crate::model::parser::Interface;

pub mod native;
pub mod builtin;
mod build_ctx;


pub trait Chip {
    fn interface(&self) -> Interface;

    fn clock(&mut self);
    fn eval(&mut self, _: &[bool]) -> Vec<bool>;
    fn chip_clone(&self) -> Box<dyn Chip>;
}

impl Clone for Box<dyn Chip> {
    fn clone(&self) -> Self {
        self.chip_clone()
    }
}