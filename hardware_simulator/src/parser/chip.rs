use crate::parser::{Builtin, Connection, Implementation, PResult, Span};

fn builtin(_: Span) -> PResult<Builtin> {

    todo!()
}

fn native(_: Span) -> PResult<Vec<Connection>> {
    todo!()
}

fn implementation(_: Span) -> PResult<Implementation> {
    todo!()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::parser::{Argument, Symbol};
//
//     #[test]
//     fn test_builtin() {
//         assert_eq!(
//             builtin(Span::new("BUILTIN Foo; CLOCKED;")),
//             Ok((
//                 Span::new(""),
//                 Builtin {
//                     name: Symbol::Name(Span::new("Foo")),
//                     clocked: Some(vec![]),
//                 }
//             ))
//         );
//         assert_eq!(
//             builtin(Span::new("BUILTIN Foo;\nCLOCKED;")),
//             Ok((
//                 Span::new(""),
//                 Builtin {
//                     name: Symbol::Name(Span::new("Foo")),
//                     clocked: Some(vec![]),
//                 }
//             ))
//         );
//         assert_eq!(
//             builtin(Span::new("BUILTIN Foo;")),
//             Ok((
//                 Span::new(""),
//                 Builtin {
//                     name: Symbol::Name(Span::new("Foo")),
//                     clocked: None
//                 }
//             ))
//         );
//         assert_eq!(
//             builtin(Span::new("BUILTIN Foo; CLOCKED a, b, c")),
//             Ok((
//                 Span::new(""),
//                 Builtin {
//                     name: Symbol::Name(Span::new("Foo")),
//                     clocked: Some(vec![
//                         Symbol::Name(Span::new("a")),
//                         Symbol::Name(Span::new("b")),
//                         Symbol::Name(Span::new("c")),
//                     ])
//                 }
//             ))
//         );
//         assert!(matches!(builtin(Span::new("BUILTIN Foo")), Err(_)));
//     }
//
//     #[test]
//     fn test_native() {
//         assert_eq!(
//             native(Span::new("PARTS:\nNand (a=a, b=b, out=n1out);\n Nand (a=n1out, b=n1out, out=out);")),
//             Ok((
//                 Span::new(""),
//                 vec![
//                     Connection {
//                         chip_name: Symbol::Name(Span::new("Nand")),
//                         inputs: vec![
//                             Argument {
//                                 internal: Symbol::Name(Span::new("a")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("b")),
//                                 external_bus: None,
//                             },
//                             Argument {
//                                 internal: Symbol::Name(Span::new("b")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("b")),
//                                 external_bus: None,
//                             },
//                             Argument {
//                                 internal: Symbol::Name(Span::new("out")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("n1out")),
//                                 external_bus: None,
//                             },
//                         ]
//                     },
//                     Connection {
//                         chip_name: Symbol::Name(Span::new("Nand")),
//                         inputs: vec![
//                             Argument {
//                                 internal: Symbol::Name(Span::new("a")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("n1out")),
//                                 external_bus: None,
//                             },
//                             Argument {
//                                 internal: Symbol::Name(Span::new("b")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("n1out")),
//                                 external_bus: None,
//                             },
//                             Argument {
//                                 internal: Symbol::Name(Span::new("out")),
//                                 internal_bus: None,
//                                 external: Symbol::Name(Span::new("out")),
//                                 external_bus: None
//                             },
//                         ]
//                     }
//                 ]
//             ))
//         )
//     }
// }
