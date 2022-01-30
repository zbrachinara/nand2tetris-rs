use crate::parser::{generic_space0, skip_comma, symbol, PResult, Pin, Span, Symbol};
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::Parser;

fn bus_declaration(arg: Span) -> PResult<u16> {
    let (remainder, size) = delimited(
        tuple((multispace0, char('['), multispace0)),
        digit1,
        tuple((multispace0, char(']'), multispace0)),
    )(arg)?;

    Ok((remainder, u16::from_str_radix(*size, 10).unwrap()))
}

fn pin_decl(arg: Span) -> PResult<Pin> {
    let (remainder, (name, bus)) = tuple((symbol, opt(complete(bus_declaration))))(arg)?;
    let (remainder, _) = skip_comma(remainder)?;

    match Symbol::try_from(name) {
        Ok(Symbol::Name(x)) => Ok((
            remainder,
            Pin {
                name: Symbol::Name(x),
                size: bus,
            },
        )),
        _ =>
        /*Err(Failure(Error {
            input: name,
            code: ErrorKind::Tag,
        })),*/
        {
            panic!()
        }
    }
}

fn headed_pin_decl<'a>(header: &'a str) -> impl FnMut(Span<'a>) -> PResult<Vec<Pin<'a>>> {
    delimited(
        tuple((generic_space0, tag(header), generic_space0)),
        many0(complete(pin_decl)),
        tuple((generic_space0, tag(";"))),
    )
}

fn in_pin_decl(arg: Span) -> PResult<Vec<Pin>> {
    headed_pin_decl("IN").parse(arg)
}

fn out_pin_decl(arg: Span) -> PResult<Vec<Pin>> {
    headed_pin_decl("OUT").parse(arg)
}

#[cfg(test)]
mod test {
    use crate::parser::test_tools::cmp_symbols;
    use super::*;

    #[test]
    fn test_bus_declaration() {
        fn check(test: PResult<u16>, exp: Result<(&str, u16), ()>) {
            match exp {
                Ok((str, num)) => match test {
                    Ok((str_test, num_test)) => {
                        assert_eq!(*str_test, str);
                        assert_eq!(num_test, num);
                    }
                    _ => panic!("{test:?}"),
                },
                Err(_) => assert!(matches!(test, Err(_))),
            }
        }

        check(bus_declaration(Span::from("[1]")), Ok(("", 1)));
        check(bus_declaration(Span::from("[5]")), Ok(("", 5)));
        check(bus_declaration(Span::from("[25]")), Ok(("", 25)));
        check(bus_declaration(Span::from("\n[\n25\n]\n")), Ok(("", 25)));
        check(
            bus_declaration(Span::from("\n[\n25\n]\nbruh")),
            Ok(("bruh", 25)),
        );
    }

    #[test]
    fn test_pin_decl() {
        {
            let res = pin_decl(Span::from("in[5]")).unwrap();
            assert_eq!(*(res.0), "");
            {
                let Pin { name, size } = res.1;
                assert_eq!(size, Some(5));
                cmp_symbols(name, Symbol::Name(Span::from("in")))
            }
        }
        {
            let res = pin_decl(Span::from("in[5], out[4]")).unwrap();
            assert_eq!(*(res.0), "out[4]");
            {
                let Pin { name, size } = res.1;
                assert_eq!(size, Some(5));
                cmp_symbols(name, Symbol::Name(Span::from("in")))
            }
        }
    }

    // #[test]
    // fn test_in_pin_decl() {
    //     check(
    //         in_pin_decl(Span::from("IN a[1], b, c[32];")),
    //         Ok((
    //             "",
    //             vec![
    //                 Pin {
    //                     name: Symbol::Name(Span::from("a")),
    //                     size: Some(1),
    //                 },
    //                 Pin {
    //                     name: Symbol::Name(Span::from("b")),
    //                     size: None,
    //                 },
    //                 Pin {
    //                     name: Symbol::Name(Span::from("c")),
    //                     size: Some(32),
    //                 }
    //             ]
    //         ))
    //     );
    //     check(
    //         in_pin_decl(Span::from("    IN a[16], b[16];")),
    //         Ok((
    //             "",
    //             vec![
    //                 Pin {
    //                     name: Symbol::Name(Span::from("a")),
    //                     size: Some(16),
    //                 },
    //                 Pin {
    //                     name: Symbol::Name(Span::from("b")),
    //                     size: Some(16),
    //                 }
    //             ]
    //         ))
    //     )
    // }
}
