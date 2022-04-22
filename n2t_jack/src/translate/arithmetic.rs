use super::common::*;
use crate::const_concat;
use crate::translate::keyword::Arithmetic;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};
use n2t_asm::parse::{Ident, Source};

const HIGH_BIT: u16 = 0b1000_0000_0000_0000;

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

const AND: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [Item::Instruction(Instruction::C {
        expr: CExpr::DAndX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

const OR: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [Item::Instruction(Instruction::C {
        expr: CExpr::DOrX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

const NOT: &[Item] = &const_concat!(
    STACK_CALL_ON_ONE,
    [Item::Instruction(Instruction::C {
        expr: CExpr::NotX(Source::Memory),
        dst: Dst::M,
        jump: JumpCondition::Never,
    })],
);

const EQ: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [
        Item::Instruction(Instruction::C {
            expr: CExpr::XMinusD(Source::Memory),
            dst: Dst::D,
            jump: JumpCondition::Never,
        }),
        Item::Instruction(Instruction::C {
            expr: CExpr::NotD,
            dst: Dst::M,
            jump: JumpCondition::Never,
        })
    ],
);

const LT: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [
        Item::Instruction(Instruction::C {
            expr: CExpr::XMinusD(Source::Memory),
            dst: Dst::D,
            jump: JumpCondition::Never,
        }),
        Item::Instruction(Instruction::A(Ident::Addr(HIGH_BIT))),
        Item::Instruction(Instruction::C {
            expr: CExpr::DAndX(Source::Register),
            dst: Dst::D,
            jump: JumpCondition::Never,
        }),
    ],
    FETCH_STACK_POINTER,
    [
        Item::Instruction(Instruction::C {
            expr: CExpr::XMinusOne(Source::Memory),
            dst: Dst::A,
            jump: JumpCondition::Never,
        }),
        Item::Instruction(Instruction::C {
            expr: CExpr::D,
            dst: Dst::M,
            jump: JumpCondition::Never,
        })
    ],
);

const GT: &[Item] = &const_concat!(
    STACK_CALL_ON_TWO,
    [
        Item::Instruction(Instruction::C {
            expr: CExpr::DMinusX(Source::Memory),
            dst: Dst::D,
            jump: JumpCondition::Never,
        }),
        Item::Instruction(Instruction::A(Ident::Addr(HIGH_BIT))),
        Item::Instruction(Instruction::C {
            expr: CExpr::DAndX(Source::Register),
            dst: Dst::D,
            jump: JumpCondition::Never,
        }),
    ],
    FETCH_STACK_POINTER,
    [
        Item::Instruction(Instruction::C {
            expr: CExpr::XMinusOne(Source::Memory),
            dst: Dst::A,
            jump: JumpCondition::Never,
        }),
        Item::Instruction(Instruction::C {
            expr: CExpr::D,
            dst: Dst::M,
            jump: JumpCondition::Never,
        })
    ],
);

impl Arithmetic {
    pub fn translate(self) -> &'static [Item] {
        match self {
            Arithmetic::Add => ADD,
            Arithmetic::Sub => SUB,
            Arithmetic::Neg => NEG,
            Arithmetic::Eq => EQ, //TODO: Define determinant bit
            Arithmetic::Gt => GT,
            Arithmetic::Lt => LT,
            Arithmetic::And => AND,
            Arithmetic::Or => OR,
            Arithmetic::Not => NOT,
        }
    }
}
