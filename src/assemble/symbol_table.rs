use std::collections::{HashMap, HashSet};
use std::ops::Index;
use crate::assemble::predefined::SYMBOLS;

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

    pub fn insert(&mut self, index: String, value: Address) {
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
