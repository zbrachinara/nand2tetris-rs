use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take, take_till};
use nom::combinator::{opt, rest};
use nom::sequence::tuple;
use nom::Parser;
use nom::{IResult, InputIter};
use std::collections::btree_map::IntoValues;
use std::collections::HashMap;
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
    internal: &'a str,
    external: &'a str,
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

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Symbol `{0}` is not a valid symbol")]
    BadSymbol(String),
}

impl<'a> TryFrom<&'a str> for Symbol<'a> {
    type Error = ParseError;

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
            Err(ParseError::BadSymbol(String::from(value)))
        }
    }
}

fn parse_arg(arg: &str) -> nom::IResult<&str, Argument> {
    let (remainder, (internal, _, external, _)) = tuple((
        // get the first name
        is_not("=").map(str::trim),
        // skip this equals sign
        tuple((take(1_usize), take_till(|c: char| !c.is_ascii_whitespace()))),
        // get the second name
        alt((is_not(","), rest)).map(str::trim),
        // skip the next comma, if it exists
        opt(tuple((
            take(1_usize),
            take_till(|c: char| !c.is_ascii_whitespace()),
        ))),
    ))
    .parse(arg)?;

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
        assert!(matches!(Symbol::try_from("u r bad"), Err(ParseError::BadSymbol(_))));

    }

    #[test]
    fn test_parse_arg() {
        assert_eq!(
            parse_arg("in = true"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
        assert_eq!(
            parse_arg("in\n=\ntrue"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true, out=false"),
            Ok((
                "out=false",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
    }
}
