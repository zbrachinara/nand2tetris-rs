use crate::parser::Span;
use nom::IResult;
use std::fmt::Debug;

pub fn check<T: PartialEq + Debug, E: PartialEq + Debug>(
    test: IResult<Span, T, E>,
    exp: IResult<&str, T, E>,
) {
    match exp {
        Ok((s, obj)) => {
            let (s_test, obj_test) = test.ok().unwrap();
            assert_eq!(*s_test, s);
            assert_eq!(obj_test, obj);
        }
        x => {
            assert_eq!(test.unwrap_err(), x.unwrap_err())
        }
    }
}
