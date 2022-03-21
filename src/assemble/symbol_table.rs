use crate::assemble::predefined::SYMBOLS;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Address {
    Rom(u16),
    Ram(u16),
}

impl Address {
    pub fn unwrap(&self) -> u16 {
        match self {
            Self::Ram(x) | Self::Rom(x) => *x,
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
            .map(|(s, x)| ((*s).to_string(), x.clone()))
            .collect::<HashMap<_, _>>();
        let value_set = map.values().cloned().collect::<HashSet<_>>();
        Self {
            map,
            value_set,
            next_ram: 0,
        }
    }

    pub fn get(&mut self, k: &str) -> Option<&Address> {
        self.map.get(k)
    }

    pub fn insert(&mut self, index: String, value: Address) -> Option<Address> {
        self.value_set.insert(value.clone());
        self.map.insert(index, value)
    }

    pub fn available_ram(&mut self) -> Option<Address> {
        let mut second_chance = false;
        while self.value_set.contains(&Address::Ram(self.next_ram)) {
            if self
                .next_ram
                .checked_add(1)
                .map(|n| self.next_ram = n)
                .is_none()
            {
                if second_chance {
                    return None;
                }
                second_chance = true;
                self.next_ram = 0;
            }
        }
        Some(Address::Ram(self.next_ram))
    }

    pub fn assign_available_ram(&mut self, name: String) -> Result<Address, String> {
        if matches!(self.map.get(name.as_str()), Some(Address::Rom(_))) {
            return Err("The given name is associated with a ROM address".to_string());
        }
        self.available_ram()
            .and_then(|address| {
                if self.insert(name, address.clone()).is_some() {
                    None
                } else {
                    Some(address)
                }
            })
            .ok_or_else(|| "Could not detect any available RAM".to_string())
    }
}
