use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Address {
    Rom(u16),
    Ram(u16),
}

pub struct SymbolTable {
    value_set: HashSet<Address>,
    map: HashMap<String, Address>,
    next_ram: u16,
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
            next_ram: 0,
        }
    }

    pub fn set(&mut self, index: String, value: Address) {
        self.map.insert(index, value.clone());
        self.value_set.insert(value);
    }

    pub fn contains(&self, value: &Address) -> bool {
        self.value_set.contains(value)
    }

    pub fn available_ram(&mut self) -> Option<Address> {
        while self.value_set.contains(&Address::Ram(self.next_ram)) {
            if self.next_ram.checked_add(1).is_none() {
                // TODO: Return to beginning to check for holes
                return None;
            }
        }
        Some(Address::Ram(self.next_ram))
    }

    pub fn assign_available_ram(&mut self, name: String) -> Result<(), String> {
        if matches!(self.map[name.as_str()], Address::Rom(_)) {
            return Err("The given name is associated with a ROM address".to_string());
        }
        self.available_ram()
            .map(|index| self.set(name, index))
            .ok_or("Could not detect any available RAM".to_string())
    }
}

impl Index<&str> for SymbolTable {
    type Output = Address;

    fn index(&self, index: &str) -> &Self::Output {
        &self.map[index]
    }
}

pub const SYMBOLS: &[(&'static str, Address)] = &[
    ("R0", Address::Ram(0)),
    ("R1", Address::Ram(1)),
    ("R2", Address::Ram(2)),
    ("R3", Address::Ram(3)),
    ("R4", Address::Ram(4)),
    ("R5", Address::Ram(5)),
    ("R6", Address::Ram(6)),
    ("R7", Address::Ram(7)),
    ("R8", Address::Ram(8)),
    ("R9", Address::Ram(9)),
    ("R10", Address::Ram(10)),
    ("R11", Address::Ram(11)),
    ("R12", Address::Ram(12)),
    ("R13", Address::Ram(13)),
    ("R14", Address::Ram(14)),
    ("R15", Address::Ram(15)),
    ("SP", Address::Ram(0)),
    ("LCL", Address::Ram(1)),
    ("ARG", Address::Ram(2)),
    ("THIS", Address::Ram(3)),
    ("THAT", Address::Ram(4)),
    ("SCREEN", Address::Ram(0x4000)),
    ("KBD", Address::Ram(0x6000)),
];
