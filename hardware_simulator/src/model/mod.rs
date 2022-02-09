use crate::model::native::NativeChip;
use parser::Interface;

mod build_ctx;
mod builtin;
mod native;
mod parser;

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

// pub enum NormalChip {
//     Native(NativeChip),
//     Builtin(Box<dyn Chip>),
// }
