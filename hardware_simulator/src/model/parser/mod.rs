use crate::Span;
use nom_supreme::error::ErrorTree;

mod chip;
mod connection;
pub(crate) mod interface;
mod channel;
mod symbols;
pub mod error;

use crate::bus_range::BusRange;
pub use chip::{chip, create_chip};
pub use interface::Interface;
pub use symbols::Symbol;

type PResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

#[derive(Debug)]
pub struct Chip<'a> {
    pub name: Span<'a>,
    pub in_pins: Vec<Channel<'a>>,
    pub out_pins: Vec<Channel<'a>>,
    pub logic: Form<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Form<'a> {
    Builtin(Builtin<'a>),
    Native(Vec<Connection<'a>>),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Builtin<'a> {
    pub name: Span<'a>,
    pub clocked: Option<Vec<Span<'a>>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Channel<'a> {
    pub name: Span<'a>,
    pub size: Option<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Connection<'a> {
    pub chip_name: Span<'a>,
    pub inputs: Vec<Argument<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    pub internal: Span<'a>,
    pub internal_bus: Option<BusRange>,
    pub external: Symbol<'a>,
    pub external_bus: Option<BusRange>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Value {
    True,
    False,
}
