use nom::bytes::complete::take_while;
use nom::character::complete::multispace0;
use nom::IResult;
use nom::sequence::delimited;
use thiserror::Error;

mod instruction;
mod pin_decl;

struct Chip {
    in_pins: Vec<String>,
    out_pins: Vec<String>,
}

struct Pin<'a> {
    name: Symbol<'a>,
    size: u16,
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

impl<'a> TryFrom<&'a str> for Symbol<'a> {
    type Error = HdlParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // a valid symbol must be in only ascii characters, as well as consisting of no whitespace
        if value.is_ascii() && value.chars().all(|c| !c.is_ascii_whitespace()) {
            Ok(if let Ok(num) = usize::from_str_radix(value, 10) {
                Symbol::Number(num)
            } else {
                match value {
                    "true" => Symbol::Value(Value::True),
                    "false" => Symbol::Value(Value::False),
                    x => Symbol::Name(x),
                }
            })
        } else {
            Err(HdlParseError::BadSymbol(value))
        }
    }
}

fn symbol(arg: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9')),
        multispace0,
    )(arg)
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
