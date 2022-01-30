use crate::parser::{generic_space0, skip_comma, symbol, Pin, Symbol, Span};
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

fn bus_declaration(arg: Span) -> IResult<Span, u16> {
    let (remainder, size) = delimited(
        tuple((multispace0, char('['), multispace0)),
        digit1,
        tuple((multispace0, char(']'), multispace0)),
    )(arg)?;

    Ok((remainder, u16::from_str_radix(*size, 10).unwrap()))
}

fn pin_decl(arg: Span) -> IResult<Span, Pin> {
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

fn headed_pin_decl(header: &str) -> impl Parser<Span, Vec<Pin>, nom::error::Error<Span>> {
    delimited(
        tuple((generic_space0, tag(header), generic_space0)),
        many0(complete(pin_decl)),
        tuple((generic_space0, tag(";"))),
    )
}

fn in_pin_decl(arg: Span) -> IResult<Span, Vec<Pin>> {
    headed_pin_decl("IN").parse(arg)
}

fn out_pin_decl(arg: Span) -> IResult<Span, Vec<Pin>> {
    headed_pin_decl("OUT").parse(arg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_declaration() {
        assert_eq!(bus_declaration(Span::from("[1]")), Ok((Span::from(""), 1)));
        assert_eq!(bus_declaration(Span::from("[5]")), Ok((Span::from(""), 5)));
        assert_eq!(bus_declaration(Span::from("[25]")), Ok((Span::from(""), 25)));
        assert_eq!(bus_declaration(Span::from("\n[\n25\n]\n")), Ok((Span::from(""), 25)));
        assert_eq!(bus_declaration(Span::from("\n[\n25\n]\nbruh")), Ok((Span::from("bruh"), 25)));
    }

    #[test]
    fn test_pin_decl() {
        assert_eq!(
            pin_decl(Span::from("in[5]")),
            Ok((
                Span::from(""),
                Pin {
                    name: Symbol::Name(Span::from("in")),
                    size: Some(5)
                }
            ))
        );
        assert_eq!(
            pin_decl(Span::from("in[5], out[4]")),
            Ok((
                Span::from("out[4]"),
                Pin {
                    name: Symbol::Name(Span::from("in")),
                    size: Some(5)
                }
            ))
        );
    }

    #[test]
    fn test_in_pin_decl() {
        assert_eq!(
            in_pin_decl(Span::from("IN a[1], b, c[32];")),
            Ok((
                Span::from(""),
                vec![
                    Pin {
                        name: Symbol::Name(Span::from("a")),
                        size: Some(1),
                    },
                    Pin {
                        name: Symbol::Name(Span::from("b")),
                        size: None,
                    },
                    Pin {
                        name: Symbol::Name(Span::from("c")),
                        size: Some(32),
                    }
                ]
            ))
        );
        assert_eq!(
            in_pin_decl(Span::from("    IN a[16], b[16];")),
            Ok((
                Span::from(""),
                vec![
                    Pin {
                        name: Symbol::Name(Span::from("a")),
                        size: Some(16),
                    },
                    Pin {
                        name: Symbol::Name(Span::from("b")),
                        size: Some(16),
                    }
                ]
            ))
        )
    }
}
