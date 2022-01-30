use crate::parser::{generic_space0, skip_comma, symbol, PResult, Pin, Span, Symbol};
use nom_supreme::tag::complete::tag;
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

fn headed_pin_decl<'a>(header: &'static str) -> impl FnMut(Span<'a>) -> PResult<Vec<Pin<'a>>> {
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
    use super::*;
    // use crate::parser::test_tools::cmp_symbols;

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

    fn check_pin_decl(test: Pin, (name, size): (&str, Option<u16>)) {
        assert!(matches!(test.name, Symbol::Name(_)));
        if let Symbol::Name(test_name) = test.name {
            assert_eq!(*test_name, name)
        } else {
            unreachable!()
        }
        assert_eq!(test.size, size);
    }

    #[test]
    fn test_pin_decl() {
        {
            let res = pin_decl(Span::from("in[5]")).unwrap();
            assert_eq!(*(res.0), "");
            check_pin_decl(res.1, ("in", Some(5)));
        }
        {
            let res = pin_decl(Span::from("in[5], out[4]")).unwrap();
            assert_eq!(*(res.0), "out[4]");
            check_pin_decl(res.1, ("in", Some(5)));
        }
    }

    #[test]
    fn test_in_pin_decl() {
        {
            let res = in_pin_decl(Span::from("IN a[1], b, c[32];")).unwrap();
            assert_eq!(*(res.0), "");
            let exp = [("a", Some(1)), ("b", None), ("c", Some(32))];
            res.1
                .into_iter()
                .zip(exp.into_iter())
                .for_each(|(test, exp)| check_pin_decl(test, exp))
        }
        {
            let res = in_pin_decl(Span::from("    IN a[16], b[16];")).unwrap();
            assert_eq!(*(res.0), "");
            let exp = [("a", Some(16)), ("b", Some(16))];
            res.1
                .into_iter()
                .zip(exp.into_iter())
                .for_each(|(test, exp)| check_pin_decl(test, exp))
        }
    }
}
