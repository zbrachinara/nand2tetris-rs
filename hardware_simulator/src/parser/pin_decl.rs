use crate::parser::{generic_space0, skip_comma, symbol, Pin, Symbol};
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

fn bus_declaration(arg: &str) -> IResult<&str, u16> {
    let (remainder, size) = delimited(
        tuple((multispace0, char('['), multispace0)),
        digit1,
        tuple((multispace0, char(']'), multispace0)),
    )(arg)?;

    Ok((remainder, u16::from_str_radix(size, 10).unwrap()))
}

fn pin_decl(arg: &str) -> IResult<&str, Pin> {
    let (remainder, (name, bus)) = tuple((symbol, opt(complete(bus_declaration))))(arg)?;
    let (remainder, _) = skip_comma(remainder)?;

    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err::*;
    match Symbol::try_from(name) {
        Ok(Symbol::Name(x)) => Ok((
            remainder,
            Pin {
                name: Symbol::Name(x),
                size: bus,
            },
        )),
        _ => Err(Failure(Error {
            input: name,
            code: ErrorKind::Tag,
        })),
    }
}

fn headed_pin_decl(header: &str) -> impl Parser<&str, Vec<Pin>, nom::error::Error<&str>> {
    delimited(
        tuple((generic_space0, tag(header), generic_space0)),
        many0(complete(pin_decl)),
        tuple((generic_space0, tag(";"))),
    )
}

fn in_pin_decl(arg: &str) -> IResult<&str, Vec<Pin>> {
    headed_pin_decl("IN").parse(arg)
}

fn out_pin_decl(arg: &str) -> IResult<&str, Vec<Pin>> {
    headed_pin_decl("OUT").parse(arg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_declaration() {
        assert_eq!(bus_declaration("[1]"), Ok(("", 1)));
        assert_eq!(bus_declaration("[5]"), Ok(("", 5)));
        assert_eq!(bus_declaration("[25]"), Ok(("", 25)));
        assert_eq!(bus_declaration("\n[\n25\n]\n"), Ok(("", 25)));
        assert_eq!(bus_declaration("\n[\n25\n]\nbruh"), Ok(("bruh", 25)));
    }

    #[test]
    fn test_pin_decl() {
        assert_eq!(
            pin_decl("in[5]"),
            Ok((
                "",
                Pin {
                    name: Symbol::Name("in"),
                    size: Some(5)
                }
            ))
        );
        assert_eq!(
            pin_decl("in[5], out[4]"),
            Ok((
                "out[4]",
                Pin {
                    name: Symbol::Name("in"),
                    size: Some(5)
                }
            ))
        );
    }

    #[test]
    fn test_in_pin_decl() {
        assert_eq!(
            in_pin_decl("IN a[1], b, c[32];"),
            Ok((
                "",
                vec![
                    Pin {
                        name: Symbol::Name("a"),
                        size: Some(1),
                    },
                    Pin {
                        name: Symbol::Name("b"),
                        size: None,
                    },
                    Pin {
                        name: Symbol::Name("c"),
                        size: Some(32),
                    }
                ]
            ))
        );
        assert_eq!(
            in_pin_decl("    IN a[16], b[16];"),
            Ok((
                "",
                vec![
                    Pin {
                        name: Symbol::Name("a"),
                        size: Some(16),
                    },
                    Pin {
                        name: Symbol::Name("b"),
                        size: Some(16),
                    }
                ]
            ))
        )
    }
}
