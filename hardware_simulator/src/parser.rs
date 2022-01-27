use lazy_static::lazy_static;
use nom::bytes::complete::take_till;
use nom::IResult;
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
    let (remainder, internal) = take_till(|c| c == '=')(arg)?;
    let (remainder, _) = take_till(|c: char| c.is_ascii_alphabetic())(remainder)?;
    let (remainder, external) = take_till(|c| c == ',')(remainder)?;

    let (internal, external) = (internal.trim(), external.trim());

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
