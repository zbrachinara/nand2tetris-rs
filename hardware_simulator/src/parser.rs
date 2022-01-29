use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_till, take_until};
use nom::character::complete::multispace0;
use nom::character::streaming::char;
use nom::combinator::{complete, opt, rest};
use nom::error::ErrorKind;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use nom::Parser;
use thiserror::Error;

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

fn trim_pair<'a, 'b>((x, y): (&'a str, &'b str)) -> (&'a str, &'b str) {
    (x.trim(), y.trim())
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

// required to start on the beginning of the bus range
fn bus_range(arg: &str) -> nom::IResult<&str, BusRange> {
    let (remainder, (start, end)) = delimited(char('['), is_not("]"), char(']'))
        .and_then(separated_pair(is_not("."), tag(".."), rest))
        .map(trim_pair)
        .parse(arg)?;

    use nom::error::Error;
    use nom::Err::*;
    match (u16::from_str_radix(start, 10), u16::from_str_radix(end, 10)) {
        (Ok(start), Ok(end)) => Ok((remainder, BusRange { start, end })),
        (Err(_), _) => Err(Failure(Error {
            input: start,
            code: ErrorKind::Tag,
        })),
        (_, Err(_)) => Err(Failure(Error {
            input: end,
            code: ErrorKind::Tag,
        })),
    }
}

fn symbol_bus(arg: &str) -> nom::IResult<&str, (&str, Option<BusRange>)> {
    tuple((
        alt((is_not(",=["), rest)).map(str::trim),
        opt(complete(bus_range)),
    ))
    .parse(arg)
}

fn parse_arg(arg: &str) -> nom::IResult<&str, Argument> {
    let (remainder, (internal, external)) = separated_pair(
        is_not("="),
        tag("="),
        alt((take_till(|c| matches!(c, ',' | ')')), rest)),
    )
    .map(trim_pair)
    .parse(arg)?;

    let ((internal, internal_bus), (external, external_bus)) =
        (symbol_bus(internal)?.1, symbol_bus(external)?.1);

    // fast forward to next argument, if it exists
    let (remainder, _) = opt(complete(tuple((
        char(','),
        take_till(|c: char| !c.is_ascii_whitespace()),
    ))))
    .parse(remainder)?;

    //TODO: Integrate these error types into the nom error types
    let (internal, external) = (
        Symbol::try_from(internal).unwrap(),
        Symbol::try_from(external).unwrap(),
    );

    IResult::Ok((
        remainder.trim_start(),
        Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        },
    ))
}

fn parse_args(arg: &str) -> nom::IResult<&str, Vec<Argument>> {
    delimited(char('('), many0(complete(parse_arg)), char(')'))(arg)
}

fn parse_instruction(_: &str) -> nom::IResult<&str, Instruction> {
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
        assert_eq!(
            bus_range("[   5   ..  10       ] and"),
            Ok((" and", BusRange { start: 5, end: 10 }))
        );
    }

    #[test]
    fn test_symbol_bus() {
        assert_eq!(
            symbol_bus("limo[1..10]"),
            Ok(("", ("limo", Some(BusRange { start: 1, end: 10 }))))
        );
        assert_eq!(
            symbol_bus("limo   [  1  .. 10  ]"),
            Ok(("", ("limo", Some(BusRange { start: 1, end: 10 }))))
        );
        assert_eq!(symbol_bus("limo   "), Ok(("", ("limo", None))));
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
                    internal_bus: None,
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("in\n=\ntrue"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("in"),
                    internal_bus: None,
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("in"),
                    internal_bus: None,
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true, out=false"),
            Ok((
                "out=false",
                Argument {
                    internal: Symbol::Name("in"),
                    internal_bus: None,
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("in[3..4]=true)"),
            Ok((
                ")",
                Argument {
                    internal: Symbol::Name("in"),
                    internal_bus: Some(BusRange { start: 3, end: 4 }),
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("in[3..4]=true, out=false"),
            Ok((
                "out=false",
                Argument {
                    internal: Symbol::Name("in"),
                    internal_bus: Some(BusRange { start: 3, end: 4 }),
                    external: Symbol::Value(Value::True),
                    external_bus: None,
                }
            ))
        );
        assert_eq!(
            parse_arg("a[9..10]=b[5..10]"),
            Ok((
                "",
                Argument {
                    internal: Symbol::Name("a"),
                    internal_bus: Some(BusRange { start: 9, end: 10 }),
                    external: Symbol::Name("b"),
                    external_bus: Some(BusRange { start: 5, end: 10 }),
                }
            ))
        )
    }

    #[test]
    fn test_parse_args() {
        assert_eq!(
            parse_args("(in=ax, out=bruh)"),
            Ok((
                "",
                vec![
                    Argument {
                        internal: Symbol::Name("in"),
                        internal_bus: None,
                        external: Symbol::Name("ax"),
                        external_bus: None,
                    },
                    Argument {
                        internal: Symbol::Name("out"),
                        internal_bus: None,
                        external: Symbol::Name("bruh"),
                        external_bus: None,
                    }
                ]
            ))
        );
    }
}
