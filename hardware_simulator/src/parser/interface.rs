use std::collections::HashMap;
use crate::parser::{BusRange, Chip};

pub struct Interface {
    pub com_in: HashMap<String, BusRange>,
    pub com_out: HashMap<String, BusRange>,
    pub seq_in: HashMap<String, BusRange>,
    pub seq_out: HashMap<String, BusRange>,
}

impl<'a> Chip<'a> {
    // defines the rules for interacting with the chip using Vec
    pub fn interface(&self) -> Interface {
        todo!()
    }
}