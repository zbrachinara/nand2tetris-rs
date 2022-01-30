use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};
use thiserror::Error;

mod connection;
mod pin_decl;
mod chip;

pub struct Chip<'a> {
    in_pins: Vec<Pin<'a>>,
    out_pins: Vec<Pin<'a>>,
    logic: Implementation<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Implementation<'a> {
    Builtin(Symbol<'a>),
    Native(Vec<Connection<'a>>),
}

pub struct Builtin<'a> {
    name: Symbol<'a>,
    clocked: bool,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Pin<'a> {
    name: Symbol<'a>,
    size: Option<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Connection<'a> {
    chip_name: Symbol<'a>,
    inputs: Vec<Argument<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    internal: Symbol<'a>,
    internal_bus: Option<BusRange>,
    external: Symbol<'a>,
    external_bus: Option<BusRange>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Value {
    True,
    False,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Symbol<'a> {
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
        take_while1(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9')),
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

fn skip_comma(arg: &str) -> IResult<&str, ()> {
    opt(complete(tuple((
        char(','),
        take_till(|c: char| !c.is_ascii_whitespace()),
    ))))
    .map(|_| ())
    .parse(arg)
}

fn generic_space1(arg: &str) -> IResult<&str, ()> {
    many0(alt((
        multispace1,
        complete(delimited(tag("/*"), take_until("*/"), tag("*/"))),
        complete(preceded(tag("//"), is_not("\n"))),
    )))
    .map(|_| ())
    .parse(arg)
}

fn generic_space0(arg: &str) -> IResult<&str, ()> {
    opt(generic_space1).map(|_| ()).parse(arg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_symbol() {
        assert_eq!(symbol("abcdef ghijkl"), Ok(("ghijkl", "abcdef")));
        assert_eq!(symbol("1234, ghijkl"), Ok((", ghijkl", "1234")));
        assert_eq!(symbol("abcd"), Ok(("", "abcd")));
        assert_eq!(symbol("AbCd"), Ok(("", "AbCd")));
        assert!(matches!(symbol(""), Err(_)))
    }

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
    fn test_generic_space0() {
        assert_eq!(generic_space0("/* // bruh */  abc"), Ok(("abc", ())));
        assert_eq!(generic_space0("//abc\ndef"), Ok(("def", ())));
        assert_eq!(generic_space0("/* word */"), Ok(("", ())));
        assert_eq!(generic_space0("/* // word */"), Ok(("", ())));
        assert_eq!(generic_space0("// /* word */"), Ok(("", ())));
        assert_eq!(generic_space0("// word"), Ok(("", ())));
        assert_eq!(generic_space0("// word\na"), Ok(("a", ())));
        assert_eq!(generic_space0("//*"), Ok(("", ())));
    }
}
