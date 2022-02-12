use crate::model::parser::Interface;
use build_ctx::ChipBuilder;
use native::NativeChip;
use std::fmt::{Debug, Formatter};

pub mod build_ctx;
mod builtin;
mod native;
mod vchip;
mod error;

pub enum Chip {
    Native(NativeChip),
    Builtin(Box<dyn ChipObject>),
}

impl Chip {
    pub fn build(name: &str, ctx: &mut ChipBuilder) -> Result<Self, ()> {
        ctx.resolve_chip(name)
    }

    pub fn interface(&self) -> Interface {
        match self {
            Chip::Native(v) => v.interface(),
            Chip::Builtin(v) => v.interface(),
        }
    }
    pub fn clock(&mut self) {
        match self {
            Chip::Native(v) => v.clock(),
            Chip::Builtin(v) => v.clock(),
        }
    }
    pub fn eval(&mut self, args: &[bool]) -> Vec<bool> {
        match self {
            Chip::Native(v) => v.eval(args),
            Chip::Builtin(v) => v.eval(args),
        }
    }
}

impl Clone for Chip {
    fn clone(&self) -> Self {
        match self {
            Chip::Native(v) => Chip::Native(v.clone()),
            Chip::Builtin(v) => Chip::Builtin(v.chip_clone()),
        }
    }
}

impl Debug for Chip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.interface().name)
    }
}

pub trait ChipObject {
    fn interface(&self) -> Interface;

    fn clock(&mut self);
    fn eval(&mut self, _: &[bool]) -> Vec<bool>;
    fn chip_clone(&self) -> Box<dyn ChipObject>;
}
