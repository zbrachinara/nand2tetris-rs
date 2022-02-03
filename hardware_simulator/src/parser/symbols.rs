use nom_supreme::error::{BaseErrorKind, ErrorTree};
use nom::combinator::{complete, opt};
use std::num::IntErrorKind;
use nom::sequence::{delimited, preceded, tuple};
use nom::character::complete::{char, multispace1};
use nom::bytes::complete::{is_not, take_till, take_until, take_while1};
use nom::multi::many0;
use nom::branch::alt;
use nom_supreme::tag::complete::tag;
use nom::Parser;
use crate::parser::{HdlParseError, PResult, Value};
use crate::Span;

#[derive(Eq, PartialEq, Debug)]
pub enum Symbol<'a> {
    Name(Span<'a>),
    Value(Value),
    Number(usize),
}

impl<'a> TryFrom<Span<'a>> for Symbol<'a> {
    type Error = Span<'a>;

    fn try_from(value: Span<'a>) -> Result<Self, Self::Error> {
        // a valid symbol must be in only ascii characters, as well as consisting of no whitespace
        if value.is_ascii() && value.chars().all(|c| !c.is_ascii_whitespace()) {
            Ok(if let Ok(num) = usize::from_str_radix(*value, 10) {
                Symbol::Number(num)
            } else {
                match *value {
                    "true" => Symbol::Value(Value::True),
                    "false" => Symbol::Value(Value::False),
                    _ => Symbol::Name(value),
                }
            })
        } else {
            Err(value)
        }
    }
}

pub fn symbol(arg: Span) -> PResult<Span> {
    spaced(take_while1(
        |c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'),
    ))(arg)
}

pub fn name(arg: Span) -> PResult<Span> {
    let (remainder, name) = symbol(arg)?;

    if matches!(
        Symbol::try_from(name),
        Ok(Symbol::Value(_) | Symbol::Number(_)) | Err(_)
    ) {
        Err(nom::Err::Error(ErrorTree::Base {
            location: arg,
            kind: BaseErrorKind::External(Box::new(HdlParseError::BadName)),
        }))
    } else {
        Ok((remainder, name))
    }
}

pub fn convert_num(span: Span) -> Result<u16, nom::Err<ErrorTree<Span>>> {
    match u16::from_str_radix(*span, 10) {
        Ok(n) => Ok(n),
        Err(e) => match e.kind() {
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                Err(nom::Err::Error(ErrorTree::Base {
                    location: span,
                    kind: BaseErrorKind::External(Box::new(HdlParseError::NumberOverflow)),
                }))
            }
            _ => Err(nom::Err::Error(ErrorTree::Base {
                location: span,
                kind: BaseErrorKind::External(Box::new(HdlParseError::NumberError)),
            })),
        },
    }
}

pub fn skip_comma(arg: Span) -> PResult<()> {
    opt(complete(tuple((
        char(','),
        take_till(|c: char| !c.is_ascii_whitespace()),
    ))))
    .map(|_| ())
    .parse(arg)
}

fn generic_space1(arg: Span) -> PResult<()> {
    many0(alt((
        multispace1,
        complete(delimited(tag("/*"), take_until("*/"), tag("*/"))),
        complete(preceded(tag("//"), is_not("\n"))),
    )))
    .map(|_| ())
    .parse(arg)
}

pub fn generic_space0(arg: Span) -> PResult<()> {
    opt(generic_space1).map(|_| ()).parse(arg)
}

pub fn spaced<'a, F: 'a, O>(inner: F) -> impl FnMut(Span<'a>) -> PResult<O>
where
    F: FnMut(Span<'a>) -> PResult<O>,
{
    delimited(generic_space0, inner, generic_space0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_symbol() {
        {
            let res = symbol(Span::new("abcdef ghijkl")).unwrap();
            assert_eq!(*(res.0), "ghijkl");
            assert_eq!(*(res.1), "abcdef");
        }
        {
            let res = symbol(Span::new("1234, ghijkl")).unwrap();
            assert_eq!(*(res.0), ", ghijkl");
            assert_eq!(*(res.1), "1234");
        }
        {
            let res = symbol(Span::new("abcd")).unwrap();
            assert_eq!(*(res.0), "");
            assert_eq!(*(res.1), "abcd");
        }
        {
            let res = symbol(Span::new("AbCd")).unwrap();
            assert_eq!(*(res.0), "");
            assert_eq!(*(res.1), "AbCd");
        }
        assert!(matches!(symbol(Span::new("")), Err(_)))
    }

    #[test]
    fn test_detect_name() {
        assert!(matches!(name(Span::new("1234")), Err(_)));
        assert!(matches!(name(Span::new("false")), Err(_)));
    }

    #[test]
    fn create_symbol() {
        assert_eq!(
            Symbol::try_from(Span::new("breh")),
            Ok(Symbol::Name(Span::new("breh")))
        );
        assert_eq!(
            Symbol::try_from(Span::new("12345")),
            Ok(Symbol::Number(12345))
        );
        assert_eq!(
            Symbol::try_from(Span::new("false")),
            Ok(Symbol::Value(Value::False))
        );
        assert!(matches!(Symbol::try_from(Span::new("u r bad")), Err(_)));
    }

    #[test]
    fn test_generic_space0() {
        fn check(test: PResult<()>, exp: Result<&str, ()>) {
            match exp {
                Ok(str) => match test {
                    Ok((rem, _)) => assert_eq!(*rem, str),
                    Err(_) => panic!("{test:?}"),
                },
                Err(_) => assert!(matches!(test, Err(_))),
            }
        }

        check(generic_space0(Span::new("/* // bruh */  abc")), Ok("abc"));
        check(generic_space0(Span::new("//abc\ndef")), Ok("def"));
        check(generic_space0(Span::new("/* // word */")), Ok(""));
        check(generic_space0(Span::new("// /* word */")), Ok(""));
        check(generic_space0(Span::new("// word")), Ok(""));
        check(generic_space0(Span::new("// word\na")), Ok("a"));
        check(generic_space0(Span::new("//*")), Ok(""));
    }
}