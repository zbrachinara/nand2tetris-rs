use super::connection::connection;
use super::pin_decl::{in_pin_decl, out_pin_decl};
use super::symbols::{name, spaced};
use super::{Builtin, Chip, Connection, HdlParseError, Implementation, PResult, Span};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::Parser;
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use nom_supreme::tag::complete::tag;

fn builtin(arg: Span) -> PResult<Builtin> {
    let (remainder, (name, clocked)) = tuple((
        spaced(delimited(tag("BUILTIN"), name, char(';'))),
        opt(spaced(delimited(
            tag("CLOCKED"),
            separated_list0(char(','), name),
            char(';'),
        ))),
    ))(arg)?;

    Ok((remainder, Builtin { name, clocked }))
}

fn native(arg: Span) -> PResult<Vec<Connection>> {
    spaced(preceded(tag("PARTS:"), many1(connection)))(arg)
}

fn implementation(arg: Span) -> PResult<Implementation> {
    let builtin = builtin(arg);
    let native = native(arg);

    if let Ok((remainder, answer)) = builtin {
        Ok((remainder, Implementation::Builtin(answer)))
    } else if let Ok((remainder, answer)) = native {
        Ok((remainder, Implementation::Native(answer)))
    } else {
        Err(nom::Err::Error(ErrorTree::Base {
            location: arg,
            kind: BaseErrorKind::External(Box::new(HdlParseError::BadImplementation)),
        }))
    }
}

pub fn chip(arg: Span) -> PResult<Chip> {
    let (remainder, (name, (in_pins, out_pins, logic))) =
        delimited(spaced(tag("CHIP")), name, spaced(tag("{")))
            .and(terminated(
                tuple((in_pin_decl, out_pin_decl, implementation)),
                spaced(tag("}")),
            ))
            .parse(arg)?;

    Ok((
        remainder,
        Chip {
            name,
            in_pins,
            out_pins,
            logic,
        },
    ))
}

pub fn create_chip(arg: Span) -> Result<Chip, nom::Err<ErrorTree<Span>>> {
    Ok(chip(arg)?.1)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::parser::symbols::Symbol;
    use crate::model::parser::Argument;

    #[test]
    fn test_builtin() {
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     CLOCKED in;}")).unwrap();
            let (remainder, Builtin { name, clocked }) = res;
            assert_eq!(*remainder, "}");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, Some(_)));
            if let Some(clocked) = clocked {
                assert_eq!(*(clocked[0]), "in");
            }
        }
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     CLOCKED in, out;")).unwrap();
            let (remainder, Builtin { name, clocked }) = res;
            assert_eq!(*remainder, "");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, Some(_)));
            if let Some(clocked) = clocked {
                assert_eq!(*(clocked[0]), "in");
                assert_eq!(*(clocked[1]), "out");
            }
        }
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     // CLOCKED in;\n}")).unwrap();
            let (remainder, Builtin { name, clocked }) = res;
            assert_eq!(*remainder, "}");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, None));
        }
    }

    #[test]
    fn test_native() {
        {
            let data = "\
    PARTS:
    Xor(a=a, b=b, out=neq1);
    Xor(a=b, b=c, out=neq2);
    Or (a=neq1, b=neq2, out=outOr);
    Not(in=outOr, out=out);
}";

            let (remainder, connections) = native(Span::new(data)).unwrap();

            assert_eq!(*remainder, "}");
            assert_eq!(connections.len(), 4);

            let checks = [
                // checking Xor(a=a, b=b, out=neq1);
                |conn: Connection| {
                    let Connection { chip_name, inputs } = conn;
                    assert_eq!(*chip_name, "Xor");
                    assert_eq!(inputs.len(), 3);

                    let checks = [
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "a");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "a")
                            }
                            assert!(matches!(external_bus, None));
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "b");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "b")
                            }
                            assert!(matches!(external_bus, None))
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "out");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "neq1")
                            }
                            assert!(matches!(external_bus, None))
                        },
                    ];

                    checks
                        .into_iter()
                        .zip(inputs.into_iter())
                        .for_each(|(check, input)| check(input));
                },
                // checking Xor(a=b, b=c, out=neq2);
                |conn: Connection| {
                    let Connection { chip_name, inputs } = conn;
                    assert_eq!(*chip_name, "Xor");
                    assert_eq!(inputs.len(), 3);

                    let checks = [
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "a");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "b")
                            }
                            assert!(matches!(external_bus, None));
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "b");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "c")
                            }
                            assert!(matches!(external_bus, None))
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "out");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "neq2")
                            }
                            assert!(matches!(external_bus, None))
                        },
                    ];

                    checks
                        .into_iter()
                        .zip(inputs.into_iter())
                        .for_each(|(check, input)| check(input));
                },
                // checking Or(a=neq1, b=neq2, out=outOr);
                |conn: Connection| {
                    let Connection { chip_name, inputs } = conn;
                    assert_eq!(*chip_name, "Or");
                    assert_eq!(inputs.len(), 3);

                    let checks = [
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "a");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "neq1")
                            }
                            assert!(matches!(external_bus, None));
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "b");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "neq2")
                            }
                            assert!(matches!(external_bus, None))
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "out");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "outOr")
                            }
                            assert!(matches!(external_bus, None))
                        },
                    ];

                    checks
                        .into_iter()
                        .zip(inputs.into_iter())
                        .for_each(|(check, input)| check(input));
                },
                // checking Not(in=outOr, out=out);
                |conn: Connection| {
                    let Connection { chip_name, inputs } = conn;
                    assert_eq!(*chip_name, "Not");
                    assert_eq!(inputs.len(), 2);

                    let checks = [
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "in");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "outOr")
                            }
                            assert!(matches!(external_bus, None));
                        },
                        |arg: Argument| {
                            let Argument {
                                internal,
                                internal_bus,
                                external,
                                external_bus,
                            } = arg;
                            assert_eq!(*internal, "out");
                            assert!(matches!(internal_bus, None));
                            assert!(matches!(external, Symbol::Name(_)));
                            if let Symbol::Name(external) = external {
                                assert_eq!(*external, "out")
                            }
                            assert!(matches!(external_bus, None))
                        },
                    ];

                    checks
                        .into_iter()
                        .zip(inputs.into_iter())
                        .for_each(|(check, input)| check(input));
                },
            ];

            checks
                .into_iter()
                .zip(connections.into_iter())
                .for_each(|(check, connection)| check(connection));
        }
    }

    #[test]
    fn test_chip_parser_success() {
        let res = chip(Span::new(include_str!("../../../../test_files/And16.hdl")));
        println!("{res:#?}");
        assert!(matches!(res, Ok(_)))
    }
}
