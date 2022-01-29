use lazy_static::lazy_static;
use thiserror::Error;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

mod instruction;

lazy_static! {
    static ref CHIP_TABLE: Arc<RwLock<HashMap<String, Chip>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

struct Chip {
    in_pins: Vec<String>,
    out_pins: Vec<String>,
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction<'a> {
    chip_name: Symbol<'a>,
    inputs: Vec<Argument<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
struct Argument<'a> {
    internal: Symbol<'a>,
    internal_bus: Option<BusRange>,
    external: Symbol<'a>,
    external_bus: Option<BusRange>,
}

#[derive(Eq, PartialEq, Debug)]
enum Value {
    True,
    False,
}

#[derive(Eq, PartialEq, Debug)]
enum Symbol<'a> {
    Name(&'a str),
    Value(Value),
    Number(usize),
}

#[derive(Debug, Eq, PartialEq)]
struct BusRange {
    start: u16,
    end: u16,
}

#[derive(Error, Debug, PartialEq)]
pub enum HdlParseError<'a> {
    #[error("Symbol `{0}` is not a valid symbol")]
    BadSymbol(&'a str),
}
