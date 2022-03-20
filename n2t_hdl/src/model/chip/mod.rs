use crate::model::parser::Interface;
use bitvec::prelude::*;
use build_ctx::ChipBuilder;
use native::NativeChip;
use std::fmt::{Debug, Display, Formatter};

pub mod build_ctx;
mod builtin;
mod error;
mod native;
mod vchip;

pub enum Chip {
    Native(NativeChip),
    Builtin(Box<dyn ChipObject>),
}

impl Chip {
    pub fn build(name: &str, ctx: &mut ChipBuilder) -> Result<Self, ()> {
        ctx.resolve_chip(name).map_err(|_| ())
    }

    pub fn is_clocked(&self) -> bool {
        match self {
            Self::Native(n) => n.is_clocked(),
            Self::Builtin(b) => b.is_clocked(),
        }
    }

    pub fn interface(&self) -> Interface {
        match self {
            Self::Native(v) => v.interface(),
            Self::Builtin(v) => v.interface(),
        }
    }
    pub fn clock(&mut self) {
        match self {
            Self::Native(v) => v.clock(),
            Self::Builtin(v) => v.clock(),
        }
    }
    pub fn eval(&mut self, args: &BitSlice) -> BitVec {
        match self {
            Self::Native(v) => v.eval(args),
            Self::Builtin(v) => v.eval(args),
        }
    }
}

impl Clone for Chip {
    fn clone(&self) -> Self {
        match self {
            Self::Native(v) => Self::Native(v.clone()),
            Self::Builtin(v) => Self::Builtin(v.chip_clone()),
        }
    }
}

impl Debug for Chip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(n) => write!(f, "{n:?}"),
            Self::Builtin(b) => write!(f, "{}", b.interface().name),
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.interface().name)
    }
}

pub trait ChipObject {
    fn interface(&self) -> Interface;

    fn is_clocked(&self) -> bool {
        self.interface().has_clocked()
    }

    fn clock(&mut self);

    fn eval(&mut self, _: &BitSlice) -> BitVec;

    fn chip_clone(&self) -> Box<dyn ChipObject>;
}
