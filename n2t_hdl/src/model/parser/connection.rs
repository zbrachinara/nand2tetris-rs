use super::symbols::{convert_num, generic_space0, name, skip_comma, spaced, symbol};
use crate::channel_range::ChannelRange;
use crate::model::parser::error::HdlParseError;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::digit1;
use nom::character::streaming::char;
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use nom::Parser;
use nom_supreme::error::BaseErrorKind;
use nom_supreme::tag::complete::tag;

use super::*;

fn bus_range(arg: Span) -> PResult<ChannelRange> {
    let (remainder, (start, end)) = spaced(delimited(char('['), is_not("]"), char(']')))
        .and_then(alt((
            separated_pair(spaced(digit1), tag(".."), spaced(digit1)),
            spaced(digit1).map(|x| (x, x)),
        )))
        .parse(arg)?;

    let (start, end) = (convert_num(start)?, convert_num(end)?);

    Ok((remainder, ChannelRange::new(start, end)))
}

fn symbol_bus(arg: Span) -> PResult<(Span, Option<ChannelRange>)> {
    tuple((symbol, opt(complete(bus_range)))).parse(arg)
}

fn single_arg(arg: Span) -> PResult<Argument> {
    let (remainder, (((internal, internal_bus), (external, external_bus)), _)) =
        separated_pair(symbol_bus, tag("="), symbol_bus)
            .and(generic_space0)
            .parse(arg)?;

    let (remainder, _) = skip_comma(remainder)?;

    let external = match Symbol::try_from(external) {
        Ok(sym) => Ok(sym),
        Err(sp) => Err(nom::Err::Error(ErrorTree::Base {
            location: sp,
            kind: BaseErrorKind::External(Box::new(HdlParseError::BadSymbol)),
        })),
    }?;

    IResult::Ok((
        remainder,
        Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        },
    ))
}

fn args(arg: Span) -> PResult<Vec<Argument>> {
    delimited(char('('), many0(single_arg), char(')'))(arg)
}

pub fn connection(arg: Span) -> PResult<Connection> {
    let (remainder, (name, args, ..)) = tuple((name, args, spaced(char(';')))).parse(arg)?;

    Ok((
        remainder,
        Connection {
            chip_name: name,
            inputs: args,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_range() {
        let test = |res: (Span, ChannelRange), excess, bus| {
            assert_eq!(*res.0, excess);
            assert_eq!(res.1, bus);
        };

        test(
            bus_range(Span::from("[0..1]")).unwrap(),
            "",
            ChannelRange::new(0, 1),
        );
        test(
            bus_range(Span::from("[5..10]")).unwrap(),
            "",
            ChannelRange::new(5, 10),
        );
        test(
            bus_range(Span::from("[5..10] and")).unwrap(),
            "and",
            ChannelRange::new(5, 10),
        );
        test(
            bus_range(Span::from("[   5   ..  10       ] and")).unwrap(),
            "and",
            ChannelRange::new(5, 10),
        );
        test(
            bus_range(Span::from("[   5\n   ..  10       ] and")).unwrap(),
            "and",
            ChannelRange::new(5, 10),
        );
        assert!(matches!(bus_range(Span::from("[ a..b]")), Err(_)));
    }

    #[test]
    fn test_symbol_bus() {
        let test = |res: (Span, (Span, Option<ChannelRange>)), bus| {
            assert_eq!(*res.0, "");
            assert_eq!(*res.1 .0, "limo");
            assert_eq!(res.1 .1, bus);
        };

        test(
            symbol_bus(Span::from("limo[1..10]")).unwrap(),
            Some(ChannelRange::new(1, 10)),
        );
        test(
            symbol_bus(Span::from("limo   [  1  .. 10  ]")).unwrap(),
            Some(ChannelRange::new(1, 10)),
        );
        test(symbol_bus(Span::from("limo   ")).unwrap(), None);
        test(symbol_bus(Span::from("limo")).unwrap(), None);
    }

    #[test]
    fn test_parse_arg() {
        let test_1 = |res: (Span, Argument)| {
            assert_eq!(*res.0, "");

            let Argument {
                internal,
                internal_bus,
                external,
                external_bus,
            } = res.1;
            assert_eq!(internal_bus, None);
            assert_eq!(external_bus, None);

            assert_eq!(*internal, "in");

            assert!(matches!(external, Symbol::Value(_)));
            if let Symbol::Value(x) = external {
                assert_eq!(x, Value::True);
            }
        };

        test_1(single_arg(Span::from("in = true")).unwrap());
        test_1(single_arg(Span::from("in\n=\ntrue")).unwrap());
        test_1(single_arg(Span::from("in=true")).unwrap());

        let test_2 = |res: (Span, Argument), excess, in_bus| {
            assert_eq!(*res.0, excess);

            let Argument {
                internal,
                internal_bus,
                external,
                external_bus,
            } = res.1;
            assert_eq!(internal_bus, in_bus);
            assert_eq!(external_bus, None);

            assert_eq!(*internal, "in");

            assert!(matches!(external, Symbol::Value(_)));
            if let Symbol::Value(x) = external {
                assert_eq!(x, Value::True);
            }
        };

        test_2(
            single_arg(Span::from("in=true, out=false")).unwrap(),
            "out=false",
            None,
        );
        test_2(
            single_arg(Span::from("in[3..4]=true)")).unwrap(),
            ")",
            Some(ChannelRange::new(3, 4)),
        );
        test_2(
            single_arg(Span::from("in[3]=true)")).unwrap(),
            ")",
            Some(ChannelRange::new(3, 3)),
        );

        let test_3 = |res: (Span, Argument), excess, in_bus, ex_bus, int, ext| {
            assert_eq!(*res.0, excess);

            let Argument {
                internal,
                internal_bus,
                external,
                external_bus,
            } = res.1;
            assert_eq!(internal_bus, in_bus);
            assert_eq!(external_bus, ex_bus);

            assert_eq!(*internal, int);

            assert!(matches!(external, Symbol::Name(_)));
            if let Symbol::Name(x) = external {
                assert_eq!(*x, ext);
            }
        };

        test_3(
            single_arg(Span::from("in[3]=out[4])")).unwrap(),
            ")",
            Some(ChannelRange::new(3, 3)),
            Some(ChannelRange::new(4, 4)),
            "in",
            "out",
        );
        test_3(
            single_arg(Span::from("a[9..10]=b[5..10]")).unwrap(),
            "",
            Some(ChannelRange::new(9, 10)),
            Some(ChannelRange::new(5, 10)),
            "a",
            "b",
        );

        {
            let res = single_arg(Span::from("in[3..4]=true, out=false")).unwrap();
            assert_eq!(*res.0, "out=false");

            let Argument {
                internal,
                internal_bus,
                external,
                external_bus,
            } = res.1;
            assert_eq!(internal_bus, Some(ChannelRange::new(3, 4)));
            assert_eq!(external_bus, None);

            assert_eq!(*internal, "in");

            assert!(matches!(external, Symbol::Value(_)));
            if let Symbol::Value(x) = external {
                assert_eq!(x, Value::True);
            }
        }
    }

    #[test]
    fn test_parse_args() {
        let res = args(Span::from("(in=ax, out=bruh)")).unwrap();
        assert_eq!(*res.0, "");

        let Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        } = &res.1[0];
        assert_eq!(internal_bus, &None);
        assert_eq!(external_bus, &None);

        assert_eq!(**internal, "in");

        assert!(matches!(external, &Symbol::Name(_)));
        if let &Symbol::Name(x) = external {
            assert_eq!(*x, "ax");
        }

        let Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        } = &res.1[1];
        assert_eq!(internal_bus, &None);
        assert_eq!(external_bus, &None);

        assert_eq!(**internal, "out");

        assert!(matches!(external, &Symbol::Name(_)));
        if let &Symbol::Name(x) = external {
            assert_eq!(*x, "bruh");
        }

        let err = args(Span::from("(in=a,"));
        assert!(matches!(err, Err(_)))
    }

    #[test]
    fn test_parse_connection() {
        let res = connection(Span::from(
            "  \n Nand (a\n[3\n..4]    =\n2, b\n[1..10]\n=  \nfalse, out=foo[6  .. 9]) ;\n  \n ",
        ))
        .unwrap();

        assert_eq!(*res.0, "");

        let Connection { chip_name, inputs } = res.1;

        assert_eq!(*chip_name, "Nand");

        let Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        } = &inputs[0];
        assert_eq!(internal_bus, &Some(ChannelRange::new(3, 4)));
        assert_eq!(external_bus, &None);

        assert_eq!(**internal, "a");

        assert!(matches!(external, &Symbol::Number(_)));
        if let &Symbol::Number(x) = external {
            assert_eq!(x, 2);
        }

        let Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        } = &inputs[1];
        assert_eq!(internal_bus, &Some(ChannelRange::new(1, 10)));
        assert_eq!(external_bus, &None);

        assert_eq!(**internal, "b");

        assert!(matches!(external, &Symbol::Value(_)));
        if let Symbol::Value(x) = external {
            assert_eq!(x, &Value::False);
        }

        let Argument {
            internal,
            internal_bus,
            external,
            external_bus,
        } = &inputs[2];
        assert_eq!(internal_bus, &None);
        assert_eq!(external_bus, &Some(ChannelRange::new(6, 9)));

        assert_eq!(**internal, "out");

        assert!(matches!(external, &Symbol::Name(_)));
        if let &Symbol::Name(x) = external {
            assert_eq!(*x, "foo");
        }
    }
}
