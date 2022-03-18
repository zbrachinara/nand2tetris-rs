mod structs;
mod space;
mod parse;

use nom::IResult;
pub use structs::*;

type PResult<'a, T> = IResult<&'a str, T>;
