use super::common::*;
use crate::const_concat;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};
use n2t_asm::parse::{Ident, Source};
use strum_macros::EnumString;

const HIGH_BIT: u16 = 0b1000_0000_0000_0000;

// const ADD: &[Item] = &const_concat!(
//     STACK_CALL_ON_TWO,
//     [Item::Instruction(Instruction::C {
//         expr: CExpr::DPlusX(Source::Memory),
//         dst: Dst::M,
//         jump: JumpCondition::Never,
//     })],
// );

const ADD: &[Item] = &n2t_asm::n2tasm!(
    {@0}
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

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    /// ```text
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// M=M-D
    /// M=!M
    /// ```
    Eq,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=D-M
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Gt,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=M-D
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Lt,
    And,
    Or,
    Not,
}
