use super::{builtin, Chip};
use crate::model::parser::Interface;
use std::collections::HashMap;

pub struct ChipBuilder {
    registered: HashMap<String, ChipInfo>,
}

pub struct ChipInfo {
    pub interface: Interface,
    pub chip: Box<dyn Chip>,
}

impl ChipBuilder {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn with_builtins(mut self) -> Self {
        self.registered
            .extend([("Nand".to_string(), builtin::nand())]);
        self
    }
}
