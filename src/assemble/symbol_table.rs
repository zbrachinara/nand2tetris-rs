use crate::assemble::predefined::SYMBOLS;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Address {
    Rom(u16),
    Ram(u16),
}

impl Address {
    pub fn unwrap(&self) -> u16 {
        match self {
            Self::Ram(x) => *x,
            Self::Rom(x) => *x,
        }
    }
}

pub struct SymbolTable {
    value_set: HashSet<Address>,
    map: HashMap<String, Address>,
    next_ram: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let map = SYMBOLS
            .iter()
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
            #[allow(clippy::question_mark)]
            if self.next_ram.checked_add(1).map(|n| self.next_ram = n).is_none() {
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
            .map(|index| self.insert(name, index))
            .ok_or_else(|| "Could not detect any available RAM".to_string())
    }
}

impl Index<&str> for SymbolTable {
    type Output = Address;

    fn index(&self, index: &str) -> &Self::Output {
        &self.map[index]
    }
}
