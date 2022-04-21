use crate::channel_range::ChannelRange;
use crate::model::chip::ChipObject;
use crate::model::parser::Interface;
use bitvec::prelude::*;
use std::iter::once;

pub fn get_builtin(name: &str) -> Option<Box<dyn ChipObject>> {
    match name {
        "Nand" => Some(Box::new(Nand)),
        _ => None,
    }
}

struct Nand;

impl ChipObject for Nand {
    fn interface(&self) -> Interface {
        Interface {
            name: "Nand".to_string(),
            com_in: [
                ("a".to_string(), ChannelRange::new(0, 0)),
                ("b".to_string(), ChannelRange::new(1, 1)),
            ]
            .into(),
            com_out: [("out".to_string(), ChannelRange::new(0, 0))].into(),
            seq_in: Default::default(),
            seq_out: Default::default(),
        }
    }

    fn clock(&mut self) {
        // nothing
    }

    fn eval(&mut self, pins: &BitSlice) -> BitVec {
        let expr = !(pins[0] && pins[1]);
        once(expr).collect()
        // vec![!(pins[0] && pins[1])]
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(Nand)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nand() {
        assert!(!Nand.is_clocked())
    }
}
