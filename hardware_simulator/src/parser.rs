use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static CHIP_TABLE: Arc<RwLock<HashMap<String, Chip>>> = Arc::new(RwLock::new(HashMap::new()));

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
