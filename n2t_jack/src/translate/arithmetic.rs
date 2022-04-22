use crate::const_concat;
use crate::translate::keyword::Arithmetic;
use n2t_asm::parse::Source;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};
use super::common::*;

impl Arithmetic {
    pub fn translate(self) -> &[Item] {
        match self {
            Arithmetic::Add => &const_concat!(
                STACK_CALL_ON_TWO,
                [Item::Instruction(Instruction::C {
                    expr: CExpr::DPlusX(Source::Memory),
                    dst: Dst::M,
                    jump: JumpCondition::Never,
                })]
            ),
            Arithmetic::Sub => &const_concat!(
                STACK_CALL_ON_TWO,
                [Item::Instruction(Instruction::C {
                    expr: CExpr::XMinusD(Source::Memory),
                    dst: Dst::M,
                    jump: JumpCondition::Never,
                })]
            ),
            Arithmetic::Neg => &const_concat!(
                FETCH_STACK_POINTER,
                DEREF_TO_A,
                DECREMENT_POINTER,
                [Item::Instruction(Instruction::C {
                    expr: CExpr::NegX(Source::Memory),
                    dst: Dst::M,
                    jump: JumpCondition::Never,
                })]
            ),
            Arithmetic::Eq => todo!(),
            Arithmetic::Gt => todo!(),
            Arithmetic::Lt => todo!(),
            Arithmetic::And => todo!(),
            Arithmetic::Or => todo!(),
            Arithmetic::Not => todo!(),
        }
    }
}
