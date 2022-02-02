use crate::model::Chip;
use crate::parser::Interface;
use crate::BusRange;
use std::collections::HashMap;
use std::iter::once;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BUILTIN: HashMap<&'static str, Box<dyn Chip + Send + Sync>> = [
        ("Nand", Box::new(Nand) as Box<dyn Chip + Send + Sync>),
    ].into_iter().collect();
}

struct Nand;
impl Chip for Nand {
    fn interface(&self) -> Interface {
        Interface {
            com_in: [
                ("a".to_string(), BusRange { start: 0, end: 0 }),
                ("b".to_string(), BusRange { start: 1, end: 1 }),
            ]
            .into_iter()
            .collect(),
            com_out: once(("out".to_string(), BusRange {start: 1, end: 1})).collect(),
            seq_in: Default::default(),
            seq_out: Default::default(),
        }
    }

    fn clock(&mut self) {
        // nothing
    }
    fn eval(&mut self, pins: &[bool]) -> Vec<bool> {
        vec![(!pins[0]) && (!pins[1])]
    }
}
