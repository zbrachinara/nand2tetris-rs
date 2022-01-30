use crate::parser::{Builtin, Connection, Implementation};
use nom::IResult;

fn builtin(_: &str) -> IResult<&str, Builtin> {
    todo!()
}

fn native(_: &str) -> IResult<&str, Vec<Connection>> {
    todo!()
}

fn implementation(_: &str) -> IResult<&str, Implementation> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{Argument, Symbol};

    #[test]
    fn test_builtin() {
        assert_eq!(
            builtin("BUILTIN Foo; CLOCKED;"),
            Ok((
                "",
                Builtin {
                    name: Symbol::Name("Foo"),
                    clocked: true
                }
            ))
        );
        assert_eq!(
            builtin("BUILTIN Foo;\nCLOCKED;"),
            Ok((
                "",
                Builtin {
                    name: Symbol::Name("Foo"),
                    clocked: true
                }
            ))
        );
        assert_eq!(
            builtin("BUILTIN Foo;"),
            Ok((
                "",
                Builtin {
                    name: Symbol::Name("Foo"),
                    clocked: false
                }
            ))
        );
        assert!(matches!(builtin("BUILTIN Foo"), Err(_)));
    }

    #[test]
    fn test_native() {
        assert_eq!(
            native("PARTS:\nNand (a=a, b=b, out=n1out);\n Nand (a=n1out, b=n1out, out=out);"),
            Ok((
                "",
                vec![
                    Connection {
                        chip_name: Symbol::Name("Nand"),
                        inputs: vec![
                            Argument {
                                internal: Symbol::Name("a"),
                                internal_bus: None,
                                external: Symbol::Name("b"),
                                external_bus: None,
                            },
                            Argument {
                                internal: Symbol::Name("b"),
                                internal_bus: None,
                                external: Symbol::Name("b"),
                                external_bus: None,
                            },
                            Argument {
                                internal: Symbol::Name("out"),
                                internal_bus: None,
                                external: Symbol::Name("n1out"),
                                external_bus: None,
                            },
                        ]
                    },
                    Connection {
                        chip_name: Symbol::Name("Nand"),
                        inputs: vec![
                            Argument {
                                internal: Symbol::Name("a"),
                                internal_bus: None,
                                external: Symbol::Name("n1out"),
                                external_bus: None,
                            },
                            Argument {
                                internal: Symbol::Name("b"),
                                internal_bus: None,
                                external: Symbol::Name("n1out"),
                                external_bus: None,
                            },
                            Argument {
                                internal: Symbol::Name("out"),
                                internal_bus: None,
                                external: Symbol::Name("out"),
                                external_bus: None
                            },
                        ]
                    }
                ]
            ))
        )
    }
}
