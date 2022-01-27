use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::combinator::{rest, success};
use nom::sequence::{terminated, tuple};
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref CHIP_TABLE: Arc<RwLock<HashMap<String, Chip>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

struct Chip {
    in_pins: Vec<String>,
    out_pins: Vec<String>,
}

struct Instruction {
    chip_name: String,
    inputs: Vec<bool>,
}

#[derive(Eq, PartialEq, Debug)]
struct Argument<'a> {
    internal: &'a str,
    external: &'a str,
}

fn parse_arg(arg: &str) -> nom::IResult<&str, Argument> {
    let (remainder, (internal, _, external)) = tuple((
        take_till(|c| c == '=').map(|s: &str| s.trim()),
        (take_till(|c: char| c.is_ascii_alphabetic())),
        (alt((take_till(|c| c == ','), rest)).map(|s: &str| s.trim())),
    ))
    .parse(arg)?;

    IResult::Ok((remainder, Argument { internal, external }))
}

fn parse_instruction(_: &str) -> nom::IResult<Instruction, &str> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_arg() {
        assert_eq!(
            parse_arg("in = true"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
    }
}
