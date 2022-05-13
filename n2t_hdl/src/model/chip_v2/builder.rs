use super::Chip;
use crate::model::parser::Interface;
use std::collections::HashMap;

pub struct ChipBuilder {
    registered: HashMap<String, ChipInfo>,
}

struct ChipInfo {
    interface: Interface,
    chip: Box<dyn Chip>,
}
