use nom::bytes::complete::{is_not, tag};
use nom::character::streaming::char;
use nom::combinator::{complete, opt};
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use nom::Parser;

use super::*;

fn bus_range(arg: &str) -> nom::IResult<&str, BusRange> {
    let (remainder, (start, end)) = delimited(
        generic_space0,
        delimited(char('['), is_not("]"), char(']')),
        generic_space0,
    )
    .and_then(separated_pair(symbol, tag(".."), symbol))
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
    tuple((symbol, opt(complete(bus_range)))).parse(arg)
}

fn parse_arg(arg: &str) -> nom::IResult<&str, Argument> {
    let (remainder, ((internal, internal_bus), (external, external_bus))) =
        separated_pair(symbol_bus, tag("="), symbol_bus).parse(arg)?;

    // fast forward to next argument, if it exists
    // let (remainder, _) = opt(complete(tuple((
    //     char(','),
    //     take_till(|c: char| !c.is_ascii_whitespace()),
    // ))))
    // .parse(remainder)?;
    let (remainder, _) = skip_comma(remainder)?;

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

fn parse_instruction(arg: &str) -> nom::IResult<&str, Instruction> {
    let (remainder, (name, args, ..)) =
        tuple((symbol, parse_args, generic_space0, char(';'), generic_space0))(arg)?;

    if let Ok(Symbol::Name(x)) = Symbol::try_from(name) {
        Ok((
            remainder,
            Instruction {
                chip_name: Symbol::Name(x),
                inputs: args,
            },
        ))
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input: name,
            code: ErrorKind::Alpha,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_range() {
        assert_eq!(bus_range("[0..1]"), Ok(("", BusRange { start: 0, end: 1 })));
        assert_eq!(
            bus_range("[5..10]"),
            Ok(("", BusRange { start: 5, end: 10 }))
        );
        assert_eq!(
            bus_range("[5..10] and"),
            Ok(("and", BusRange { start: 5, end: 10 }))
        );
        assert_eq!(
            bus_range("[   5   ..  10       ] and"),
            Ok(("and", BusRange { start: 5, end: 10 }))
        );
        assert_eq!(
            bus_range("[   5\n  ..  10       ] and"),
            Ok(("and", BusRange { start: 5, end: 10 }))
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

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction(
                "Nand (a\n[3\n..4]    =\n2, b\n[1..10]\n=  \nfalse, out=foo[6  ..  9])   ;"
            ),
            Ok((
                "",
                Instruction {
                    chip_name: Symbol::Name("Nand"),
                    inputs: vec![
                        Argument {
                            internal: Symbol::Name("a"),
                            internal_bus: Some(BusRange { start: 3, end: 4 }),
                            external: Symbol::Number(2),
                            external_bus: None,
                        },
                        Argument {
                            internal: Symbol::Name("b"),
                            internal_bus: Some(BusRange { start: 1, end: 10 }),
                            external: Symbol::Value(Value::False),
                            external_bus: None,
                        },
                        Argument {
                            internal: Symbol::Name("out"),
                            internal_bus: None,
                            external: Symbol::Name("foo"),
                            external_bus: Some(BusRange { start: 6, end: 9 }),
                        }
                    ]
                }
            ))
        )
    }
}
