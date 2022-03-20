mod cinstr;
mod parse;
mod space;
mod structs;

use nom::IResult;
pub use structs::*;

pub fn program(program: &str) -> Result<Program, nom::Err<nom::error::Error<&str>>> {
    parse::program(program).map(|(_, program)| program)
}

type PResult<'a, T> = IResult<&'a str, T>;


