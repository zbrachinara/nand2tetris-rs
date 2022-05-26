use crate::Span;
use nom_supreme::error::ErrorTree;

mod channel;
mod chip;
mod connection;
pub mod error;
pub(crate) mod interface;
mod symbols;

use crate::channel_range::ChannelRange;
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
    pub internal_bus: Option<ChannelRange>,
    pub external: Symbol<'a>,
    pub external_bus: Option<ChannelRange>,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Value {
    True,
    False,
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Self::True => true,
            Self::False => false,
        }
    }
}
