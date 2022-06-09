use super::{builder::ChipInfo, Chip};
use crate::model::parser::{interface::ChannelPin, Interface};
use bitvec::prelude::*;

#[derive(Clone)]
struct Nand {
    bit: bool,
}

impl Nand {
    fn new() -> Self {
        Self { bit: false }
    }
}

impl Chip for Nand {
    fn eval(&mut self, args: &BitSlice) -> BitVec {
        self.bit = !(args[0] && args[1]);
        BitVec::repeat(self.bit, 1)
    }
    fn boxed_clone(&self) -> Box<dyn Chip> {
        Box::new(self.clone())
    }
    fn clock(&mut self, _: &BitSlice) -> BitVec {
        BitVec::repeat(self.bit, 1)
    }
}

pub fn nand() -> ChipInfo {
    ChipInfo {
        interface: Interface {
            name: "Nand".to_string(),
            map: [
                ("a".to_string(), ChannelPin::ComIn((0..=0).into())),
                ("b".to_string(), ChannelPin::ComIn((1..=1).into())),
                ("out".to_string(), ChannelPin::ComOut((0..=0).into())),
            ]
            .into(),
        },
        chip: Box::new(Nand::new()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn nand() {
        let mut nand = Nand::new();
        assert_eq!(nand.eval(bits!(0, 0)), bits!(1));
        assert_eq!(nand.eval(bits!(0, 1)), bits!(1));
        assert_eq!(nand.eval(bits!(1, 0)), bits!(1));
        assert_eq!(nand.eval(bits!(1, 1)), bits!(0));
    }
}
