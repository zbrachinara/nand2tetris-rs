use crate::model::chip::ChipObject;
use crate::model::parser::interface::ChannelPin;
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
            map: [
                ("a".to_string(), ChannelPin::ComIn((0..=0).into())),
                ("b".to_string(), ChannelPin::ComIn((1..=1).into())),
                ("out".to_string(), ChannelPin::ComOut((0..=0).into())),
            ]
            .into(),
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
