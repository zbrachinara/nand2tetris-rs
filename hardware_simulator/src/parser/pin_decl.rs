use crate::parser::{symbol, Pin, Symbol, skip_comma};
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{complete, opt};
use nom::sequence::{delimited, tuple};
use nom::IResult;

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
}
