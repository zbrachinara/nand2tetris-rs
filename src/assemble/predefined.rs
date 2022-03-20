use std::collections::{HashMap, HashSet};
use std::ops::Index;

pub struct SymbolTable {
    next_available: u16,
    value_set: HashSet<u16>,
    map: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let map = SYMBOLS
            .into_iter()
            .map(|(s, x)| (s.to_string(), x.clone()))
            .collect::<HashMap<_, _>>();
        let value_set = map.values().cloned().collect::<HashSet<_>>();
        Self {
            map,
            value_set,
            next_available: 0,
        }
    }

    pub fn set(&mut self, index: String, value: u16) {
        self.map.insert(index, value);
        self.value_set.insert(value);
    }

    pub fn contains(&self, value: &u16) -> bool {
        self.value_set.contains(value)
    }

    pub fn next_available(&mut self) -> Option<u16> {
        while self.value_set.contains(&self.next_available) {
            if self.next_available.checked_add(1).is_none() {
                return None;
            }
        }
        Some(self.next_available)
    }

    pub fn assign_next_available(&mut self, name: String) -> bool {
        self.next_available().map(|index| {
            self.set(name, index)
        }).is_some()
    }
}

impl Index<&str> for SymbolTable {
    type Output = u16;

    fn index(&self, index: &str) -> &Self::Output {
        &self.map[index]
    }
}

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
