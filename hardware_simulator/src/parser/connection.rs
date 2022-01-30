use nom::bytes::complete::{is_not, tag};
use nom::character::complete::digit1;
use nom::character::streaming::char;
use nom::combinator::{complete, opt};
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use nom::Parser;
use nom_supreme::error::BaseErrorKind;

use super::*;

fn bus_range(arg: Span) -> PResult<BusRange> {
    let (remainder, (start, end)) = delimited(
        generic_space0,
        delimited(char('['), is_not("]"), char(']')),
        generic_space0,
    )
    .and_then(alt((
        separated_pair(symbol, tag(".."), symbol),
        digit1.map(|x| (x, x)),
    )))
    .parse(arg)?;

    match (
        u16::from_str_radix(*start, 10),
        u16::from_str_radix(*end, 10),
    ) {
        (Ok(start), Ok(end)) => Ok((remainder, BusRange { start, end })),
        (Err(e), _) => Err(nom::Err::Error(ErrorTree::Base {
            location: start,
            kind: BaseErrorKind::External(Box::new(e)),
        })),
        (_, Err(e)) => Err(nom::Err::Error(ErrorTree::Base {
            location: end,
            kind: BaseErrorKind::External(Box::new(e)),
        })),
    }
}

fn symbol_bus(arg: Span) -> PResult<(Span, Option<BusRange>)> {
    tuple((symbol, opt(complete(bus_range)))).parse(arg)
}

fn parse_arg(arg: Span) -> PResult<Argument> {
    let (remainder, (((internal, internal_bus), (external, external_bus)), _)) =
        separated_pair(symbol_bus, tag("="), symbol_bus)
            .and(generic_space0)
            .parse(arg)?;

    let (remainder, _) = skip_comma(remainder)?;

    //TODO: Integrate these error types into the nom error types
    let (internal, external) = (
        Symbol::try_from(internal).unwrap(),
        Symbol::try_from(external).unwrap(),
    );

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

fn parse_args(arg: Span) -> PResult<Vec<Argument>> {
    delimited(char('('), many0(complete(parse_arg)), char(')'))(arg)
}

pub fn parse_connection(arg: Span) -> PResult<Connection> {
    let (remainder, (name, args, ..)) = tuple((
        symbol,
        parse_args,
        generic_space0,
        char(';'),
        generic_space0,
    ))
    .parse(arg)?;

    if let Ok(Symbol::Name(x)) = Symbol::try_from(name) {
        Ok((
            remainder,
            Connection {
                chip_name: Symbol::Name(x),
                inputs: args,
            },
        ))
    } else {
        Err(nom::Err::Error(ErrorTree::Base {
            location: name,
            kind: BaseErrorKind::Kind(ErrorKind::Alpha),
        }))
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_bus_range() {
//         {
//             let res = bus_range(Span::from("[0..1]"));
//             assert_eq!(
//                 bus_range(Span::from("[0..1]")),
//                 Ok((Span::from(""), BusRange { start: 0, end: 1 }))
//             );
//         }
//
//         assert_eq!(
//             bus_range(Span::from("[5..10]")),
//             Ok((Span::from(""), BusRange { start: 5, end: 10 }))
//         );
//         assert_eq!(
//             bus_range(Span::from("[5..10] and")),
//             Ok((Span::from("and"), BusRange { start: 5, end: 10 }))
//         );
//         assert_eq!(
//             bus_range(Span::from("[   5   ..  10       ] and")),
//             Ok((Span::from("and"), BusRange { start: 5, end: 10 }))
//         );
//         assert_eq!(
//             bus_range(Span::from("[   5\n  ..  10       ] and")),
//             Ok((Span::from("and"), BusRange { start: 5, end: 10 }))
//         );
//     }
//
//     #[test]
//     fn test_symbol_bus() {
//         assert_eq!(
//             symbol_bus(Span::from("limo[1..10]")),
//             Ok((
//                 Span::from(""),
//                 (Span::from("limo"), Some(BusRange { start: 1, end: 10 }))
//             ))
//         );
//         assert_eq!(
//             symbol_bus(Span::from("limo   [  1  .. 10  ]")),
//             Ok((
//                 Span::from(""),
//                 (Span::from("limo"), Some(BusRange { start: 1, end: 10 }))
//             ))
//         );
//         assert_eq!(
//             symbol_bus(Span::from("limo   ")),
//             Ok((Span::from(""), (Span::from("limo"), None)))
//         );
//         assert_eq!(
//             symbol_bus(Span::from("limo")),
//             Ok((Span::from(""), (Span::from("limo"), None)))
//         )
//     }
//
//     #[test]
//     fn test_parse_arg() {
//         assert_eq!(
//             parse_arg(Span::from("in = true")),
//             Ok((
//                 Span::from(""),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: None,
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in\n=\ntrue")),
//             Ok((
//                 Span::from(""),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: None,
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in=true")),
//             Ok((
//                 Span::from(""),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: None,
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in=true, out=false")),
//             Ok((
//                 Span::from("out=false"),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: None,
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in[3..4]=true)")),
//             Ok((
//                 Span::from(")"),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: Some(BusRange { start: 3, end: 4 }),
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in[3]=true)")),
//             Ok((
//                 Span::from(")"),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: Some(BusRange { start: 3, end: 3 }),
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in[3]=out[4])")),
//             Ok((
//                 Span::from(")"),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: Some(BusRange { start: 3, end: 3 }),
//                     external: Symbol::Name(Span::from("out")),
//                     external_bus: Some(BusRange { start: 4, end: 4 }),
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("in[3..4]=true, out=false")),
//             Ok((
//                 Span::from("out=false"),
//                 Argument {
//                     internal: Symbol::Name(Span::from("in")),
//                     internal_bus: Some(BusRange { start: 3, end: 4 }),
//                     external: Symbol::Value(Value::True),
//                     external_bus: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             parse_arg(Span::from("a[9..10]=b[5..10]")),
//             Ok((
//                 Span::from(""),
//                 Argument {
//                     internal: Symbol::Name(Span::from("a")),
//                     internal_bus: Some(BusRange { start: 9, end: 10 }),
//                     external: Symbol::Name(Span::from("b")),
//                     external_bus: Some(BusRange { start: 5, end: 10 }),
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_parse_args() {
//         assert_eq!(
//             parse_args(Span::from("(in=ax, out=bruh)")),
//             Ok((
//                 Span::from(""),
//                 vec![
//                     Argument {
//                         internal: Symbol::Name(Span::from("in")),
//                         internal_bus: None,
//                         external: Symbol::Name(Span::from("ax")),
//                         external_bus: None,
//                     },
//                     Argument {
//                         internal: Symbol::Name(Span::from("out")),
//                         internal_bus: None,
//                         external: Symbol::Name(Span::from("bruh")),
//                         external_bus: None,
//                     }
//                 ]
//             ))
//         );
//     }
//
//     #[test]
//     fn test_parse_connection() {
//         assert_eq!(
//             parse_connection(
//                 Span::from("  \n Nand (a\n[3\n..4]    =\n2, b\n[1..10]\n=  \nfalse, out=foo[6  .. 9]) ;\n  \n ")
//             ),
//             Ok((
//                 Span::from(""),
//                 Connection {
//                     chip_name: Symbol::Name(Span::from("Nand")),
//                     inputs: vec![
//                         Argument {
//                             internal: Symbol::Name(Span::from("a")),
//                             internal_bus: Some(BusRange { start: 3, end: 4 }),
//                             external: Symbol::Number(2),
//                             external_bus: None,
//                         },
//                         Argument {
//                             internal: Symbol::Name(Span::from("b")),
//                             internal_bus: Some(BusRange { start: 1, end: 10 }),
//                             external: Symbol::Value(Value::False),
//                             external_bus: None,
//                         },
//                         Argument {
//                             internal: Symbol::Name(Span::from("out")),
//                             internal_bus: None,
//                             external: Symbol::Name(Span::from("foo")),
//                             external_bus: Some(BusRange { start: 6, end: 9 }),
//                         }
//                     ]
//                 }
//             ))
//         )
//     }
// }
