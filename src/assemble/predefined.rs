use std::collections::HashMap;

pub const SYMBOLS: &[(&'static str, u16)] = &[
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("SCREEN", 0x4000),
    ("KBD", 0x6000),
];

pub fn new_symbol_table() -> HashMap<String, u16> {
    SYMBOLS
        .into_iter()
        .map(|(s, x)| (s.to_string(), x.clone()))
        .collect()
}
