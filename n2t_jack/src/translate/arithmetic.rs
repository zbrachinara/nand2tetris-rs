use super::common::*;
use crate::const_concat;
use crate::translate::keyword::Arithmetic;
use n2t_asm::parse::Source;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};

const ADD: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [Item::Instruction(Instruction::C {
        expr: CExpr::DPlusX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

const SUB: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [Item::Instruction(Instruction::C {
        expr: CExpr::DMinusX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

const NEG: &[Item] = &const_concat!(
    STACK_CALL_ON_ONE,
    [Item::Instruction(Instruction::C {
        expr: CExpr::NegX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

impl Arithmetic {
    pub fn translate(self) -> &'static [Item] {
        match self {
            Arithmetic::Add => ADD,
            Arithmetic::Sub => SUB,
            Arithmetic::Neg => NEG,
            Arithmetic::Eq => todo!(),
            Arithmetic::Gt => todo!(),
            Arithmetic::Lt => todo!(),
            Arithmetic::And => todo!(),
            Arithmetic::Or => todo!(),
            Arithmetic::Not => todo!(),
        }
    }
}
