use std::collections::HashMap;

use super::Chip;
use crate::model::parser::Interface;
use bitvec::prelude::*;

struct Nand;
impl Chip for Nand {
    fn clock(&mut self) {}

    fn eval(&mut self, args: &BitSlice) -> BitVec {
        [!(args[0] && args[1])].into_iter().collect()
    }
}

pub fn nand() -> (Box<dyn Chip>, Interface) {
    (
        Box::new(Nand),
        Interface {
            name: "Nand".to_string(),
            com_in: [
                ("a".to_string(), (0..=0).into()),
                ("b".to_string(), (1..=1).into()),
            ]
            .into_iter()
            .collect(),
            com_out: [("out".to_string(), (0..=0).into())].into_iter().collect(),
            seq_in: HashMap::new(),
            seq_out: HashMap::new(),
        },
    )
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
