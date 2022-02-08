use super::symbols::*;
use super::*;
use nom::character::complete::{char, digit1};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{Err, Parser};
use nom_supreme::tag::complete::tag;

fn bus_declaration(arg: Span) -> PResult<Span> {
    let (remainder, size) = delimited(spaced(char('[')), symbol, spaced(char(']')))(arg)?;
    Ok((remainder, size))
}

fn pin_decl(arg: Span) -> PResult<Pin> {
    let (remainder, (name, size)) = tuple((name, opt(bus_declaration)))(arg)?;
    let (remainder, _) = skip_comma(remainder)?;

    match size {
        Some(size) => Ok((
            remainder,
            Pin {
                name,
                size: Some(convert_num(size)?),
            },
        )),
        None => Ok((remainder, Pin { name, size: None })),
    }
}

fn headed_pin_decl<'a>(header: &'static str) -> impl FnMut(Span<'a>) -> PResult<Vec<Pin<'a>>> {
    delimited(
        spaced(tag(header)),
        many0(complete(pin_decl)),
        tuple((generic_space0, tag(";"))),
    )
}

pub fn in_pin_decl(arg: Span) -> PResult<Vec<Pin>> {
    headed_pin_decl("IN").parse(arg)
}

pub fn out_pin_decl(arg: Span) -> PResult<Vec<Pin>> {
    headed_pin_decl("OUT").parse(arg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_declaration() {
        fn check(test: PResult<Span>, exp: Result<(&str, &str), ()>) {
            match exp {
                Ok((str, num)) => match test {
                    Ok((str_test, num_test)) => {
                        assert_eq!(*str_test, str);
                        assert_eq!(*num_test, num);
                    }
                    _ => panic!("{test:?}"),
                },
                Err(_) => assert!(matches!(test, Err(_))),
            }
        }

        check(bus_declaration(Span::from("[1]")), Ok(("", "1")));
        check(bus_declaration(Span::from("[5]")), Ok(("", "5")));
        check(bus_declaration(Span::from("[25]")), Ok(("", "25")));
        check(bus_declaration(Span::from("\n[\n25\n]\n")), Ok(("", "25")));
        check(
            bus_declaration(Span::from("\n[\n25\n]\nbruh")),
            Ok(("bruh", "25")),
        );
        check(bus_declaration(Span::from("[abcdef]")), Ok(("", "abcdef")));
    }

    fn check_pin_decl(test: Pin, (name, size): (&str, Option<u16>)) {
        assert_eq!(*test.name, name);
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
        {
            let res = pin_decl(Span::from("in[abc]"));
            println!("{res:?}");
            assert!(matches!(res, Err(_)))
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
