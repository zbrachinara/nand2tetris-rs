use crate::parser::{Span, Symbol};
use nom::IResult;
use std::fmt::Debug;

// pub fn cmp_symbols(test: Symbol, exp: Symbol) {
//     match exp {
//         Symbol::Name(sp) => {
//             matches!(test, Symbol::Name(_));
//             if let Symbol::Name(sp_test) = test {
//                 cmp_spans(sp_test, sp)
//             } else {
//                 unreachable!()
//             }
//         }
//         _ => {
//             assert_eq!(test, exp)
//         }
//     }
// }

// pub fn cmp_spans(test: Span, exp: Span) {
//     assert_eq!(*test, *exp)
// }

// pub fn check<T: PartialEq + Debug, E: PartialEq + Debug>(
//     test: IResult<Span, T, E>,
//     exp: IResult<&str, T, E>,
// ) {
//     match exp {
//         Ok((s, obj)) => {
//             let (s_test, obj_test) = test.ok().unwrap();
//             assert_eq!(*s_test, s);
//             assert_eq!(obj_test, obj);
//         }
//         x => {
//             assert_eq!(test.unwrap_err(), x.unwrap_err())
//         }
//     }
// }
