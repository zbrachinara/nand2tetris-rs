use crate::{BusRange, Span};
use nom_supreme::error::ErrorTree;
use thiserror::Error;

mod chip;
mod connection;
mod interface;
mod pin_decl;
mod symbols;

pub use chip::chip;
pub use interface::Interface;
use symbols::Symbol;

type PResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

#[derive(Debug)]
pub struct Chip<'a> {
    pub name: Span<'a>,
    pub in_pins: Vec<Pin<'a>>,
    pub out_pins: Vec<Pin<'a>>,
    pub logic: Implementation<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Implementation<'a> {
    Builtin(Builtin<'a>),
    Native(Vec<Connection<'a>>),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Builtin<'a> {
    pub name: Span<'a>,
    pub clocked: Option<Vec<Span<'a>>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Pin<'a> {
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

#[derive(Error, Debug, PartialEq)]
pub enum HdlParseError {
    #[error("Not a valid symbol")]
    BadSymbol,
    #[error("Name is not correct (Must not be a number or literal)")]
    BadName,
    #[error("Number is too large")]
    NumberOverflow,
    #[error("A problem occurred when trying to parse this number")]
    NumberError,
    #[error("Could not deduce a given implementation")]
    BadImplementation,
}
