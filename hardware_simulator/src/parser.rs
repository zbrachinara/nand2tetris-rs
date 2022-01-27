use lazy_static::lazy_static;
use nom::bytes::complete::take_till;
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

fn parse_instruction() -> Instruction {
    todo!()
}
