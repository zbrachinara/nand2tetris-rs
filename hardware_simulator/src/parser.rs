use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_till};
use nom::character::streaming::char;
use nom::combinator::{opt, rest, value};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::{delimited, pair, separated_pair, tuple};
use nom::Parser;
use nom::{IResult, InputIter};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use thiserror::Error;

lazy_static! {
    static ref CHIP_TABLE: Arc<RwLock<HashMap<String, Chip>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

struct Chip {
    in_pins: Vec<String>,
    out_pins: Vec<String>,
}

struct Instruction {
    chip_name: String,
    inputs: Vec<bool>,
}

#[derive(Eq, PartialEq, Debug)]
struct Argument<'a> {
    internal: Symbol<'a>,
    external: Symbol<'a>,
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

pub fn drop_err<I: Clone, O, E: ParseError<I>, F>(mut f: F) -> impl FnMut(I) -> IResult<I, Option<O>, E>
    where
        F: Parser<I, O, E>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(_) => Ok((i, None)),
        }
    }
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

fn bus_range(arg: &str) -> nom::IResult<&str, BusRange> {
    let (remainder, (start, end)) = delimited(char('['), is_not("]"), char(']'))
        .and_then(separated_pair(is_not("."), tag(".."), rest))
        .parse(arg)?;

    use nom::error::Error;
    use nom::Err::*;
    match (u16::from_str_radix(start, 10), u16::from_str_radix(end, 10)) {
        (Ok(start), Ok(end)) => Ok((remainder, BusRange { start, end })),
        (Err(e), _) => Err(Failure(Error {
            input: start,
            code: ErrorKind::Tag,
        })),
        (_, Err(e)) => Err(Failure(Error {
            input: end,
            code: ErrorKind::Tag,
        })),
    }
}

fn symbol_bus(arg: &str) -> nom::IResult<&str, (&str, Option<BusRange>)> {
    tuple((alt((is_not(",=["), rest)), drop_err(bus_range))).parse(arg)
}

fn parse_arg(arg: &str) -> nom::IResult<&str, Argument> {
    let (remainder, (internal, external)) = take_till(|c: char| c == ',')
        .and_then(separated_pair(is_not("="), tag("="), rest))
        .map(|(x, y): (&str, &str)| (x.trim(), y.trim()))
        .parse(arg)?;

    // fast forward to next argument, if it exists
    let (remainder, _) = opt(tuple((
        take(1_usize),
        take_till(|c: char| !c.is_ascii_whitespace()),
    )))
    .parse(remainder)?;

    //TODO: Integrate these error types into the nom error types
    let (internal, external) = (
        Symbol::try_from(internal).unwrap(),
        Symbol::try_from(external).unwrap(),
    );

    IResult::Ok((remainder.trim_start(), Argument { internal, external }))
}

fn parse_instruction(_: &str) -> nom::IResult<Instruction, &str> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_symbol() {
        assert_eq!(Symbol::try_from("breh"), Ok(Symbol::Name("breh")));
        assert_eq!(Symbol::try_from("12345"), Ok(Symbol::Number(12345)));
        assert_eq!(Symbol::try_from("false"), Ok(Symbol::Value(Value::False)));
        assert!(matches!(
            Symbol::try_from("u r bad"),
            Err(HdlParseError::BadSymbol(_))
        ));
    }

    #[test]
    fn test_bus_range() {
        assert_eq!(bus_range("[0..1]"), Ok(("", BusRange { start: 0, end: 1 })));
        assert_eq!(
            bus_range("[5..10]"),
            Ok(("", BusRange { start: 5, end: 10 }))
        );
        assert_eq!(
            bus_range("[5..10] and"),
            Ok((" and", BusRange { start: 5, end: 10 }))
        );
    }

    #[test]
    fn test_symbol_bus() {
        assert_eq!(
            symbol_bus("limo[1..10]"),
            Ok(("", ("limo", Some(BusRange { start: 1, end: 10 }))))
        );
        assert_eq!(symbol_bus("limo"), Ok(("", ("limo", None))))
    }

    #[test]
    fn test_parse_arg() {
        assert_eq!(
            parse_arg("in = true"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("in"),
                    external: Symbol::Value(Value::True)
                }
            ))
        );
        assert_eq!(
            parse_arg("in\n=\ntrue"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("in"),
                    external: Symbol::Value(Value::True)
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("in"),
                    external: Symbol::Value(Value::True)
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true, out=false"),
            Ok((
                "out=false",
                Argument {
                    internal: Symbol::Name("in"),
                    external: Symbol::Value(Value::True)
                }
            ))
        );
    }
}
