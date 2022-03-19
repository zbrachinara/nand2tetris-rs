mod cinstr;
mod parse;
mod space;
mod structs;

use nom::IResult;
pub use structs::*;
pub use parse::program;

type PResult<'a, T> = IResult<&'a str, T>;


