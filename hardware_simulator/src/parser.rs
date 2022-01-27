

struct Chip {
    in_pins: Vec<String>,
    out_pins: Vec<String>,
}

struct Instruction {
    chip_name: String,
    inputs: Vec<bool>
}
