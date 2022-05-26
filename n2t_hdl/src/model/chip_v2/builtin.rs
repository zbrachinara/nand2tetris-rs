use super::{builder::ChipInfo, Chip};
use crate::model::parser::{interface::ChannelPin, Interface};
use bitvec::prelude::*;

struct Nand;
impl Chip for Nand {
    fn clock(&mut self) {}
    fn eval(&mut self, args: &BitSlice) -> BitVec {
        [!(args[0] && args[1])].into_iter().collect()
    }
    fn boxed_clone(&self) -> Box<dyn Chip> {
        Box::new(Nand)
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
            .into(), // com_in: [
                     //     ("a".to_string(), (0..=0).into()),
                     //     ("b".to_string(), (1..=1).into()),
                     // ]
                     // .into_iter()
                     // .collect(),
                     // com_out: [("out".to_string(), (0..=0).into())].into_iter().collect(),
                     // seq_in: HashMap::new(),
                     // seq_out: HashMap::new(),
        },
        chip: Box::new(Nand),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn nand() {
        assert_eq!(Nand.eval(bits!(0, 0)), bits!(1));
        assert_eq!(Nand.eval(bits!(0, 1)), bits!(1));
        assert_eq!(Nand.eval(bits!(1, 0)), bits!(1));
        assert_eq!(Nand.eval(bits!(1, 1)), bits!(0));
    }
}
