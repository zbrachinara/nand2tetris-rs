use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take, take_till};
use nom::combinator::{opt, rest};
use nom::sequence::tuple;
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
    let (remainder, (internal, _, external, _)) = tuple((
        // get the first name
        is_not("=").map(str::trim),
        // skip this comma
        tuple((take(1_usize), take_till(|c: char| !c.is_ascii_whitespace()))),
        // get the second name
        alt((is_not(","), rest)).map(str::trim),
        // skip the next comma, if it exists
        opt(tuple((
            take(1_usize),
            take_till(|c: char| !c.is_ascii_whitespace()),
        ))),
    ))
    .parse(arg)?;

    IResult::Ok((remainder.trim_start(), Argument { internal, external }))
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
        assert_eq!(
            parse_arg("in\n=\ntrue"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true"),
            Ok((
                "",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
        assert_eq!(
            parse_arg("in=true, out=false"),
            Ok((
                "out=false",
                Argument {
                    internal: "in",
                    external: "true"
                }
            ))
        );
    }
}
