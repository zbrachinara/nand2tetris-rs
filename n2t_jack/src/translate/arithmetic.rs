use crate::const_concat;
use crate::translate::keyword::Arithmetic;
use n2t_asm::parse::Source;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};

/// Instruction which loads the stack pointer into M
pub const FETCH_STACK_POINTER: [Item; 1] = [Item::Instruction(Instruction::C {
    expr: CExpr::Zero,
    dst: Dst::A,
    jump: JumpCondition::Never,
})];

pub const DECREMENT_MEM: [Item; 1] = [Item::Instruction(Instruction::C {
    expr: CExpr::XMinusOne(Source::Memory),
    dst: Dst::M,
    jump: JumpCondition::Never,
})];

/// Decrements the hardware pointer (A)
pub const DECREMENT_POINTER: [Item; 1] = [Item::Instruction(Instruction::C {
    expr: CExpr::XMinusOne(Source::Register),
    dst: Dst::A,
    jump: JumpCondition::Never,
})];

/// Moves the contents of M into D
pub const DEREF_TO_D: [Item; 1] = [Item::Instruction(Instruction::C {
    expr: CExpr::X(Source::Memory),
    dst: Dst::D,
    jump: JumpCondition::Never,
})];

/// Moves the contents of M into A
pub const DEREF_TO_A: [Item; 1] = [Item::Instruction(Instruction::C {
    expr: CExpr::X(Source::Memory),
    dst: Dst::A,
    jump: JumpCondition::Never,
})];

/// pops the stack
/// The new stack pointer will be stored to A, and also points to the old stack top value
pub const STACK_POP: [Item; 3] = const_concat!(FETCH_STACK_POINTER, DECREMENT_MEM, DEREF_TO_A);

impl Arithmetic {
    pub fn translate(self) -> impl Iterator<Item = Item> {
        match self {
            Arithmetic::Add => const_concat!(
                STACK_POP,
                DEREF_TO_D,
                DECREMENT_POINTER,
                [Item::Instruction(Instruction::C {
                    expr: CExpr::DPlusX(Source::Memory),
                    dst: Dst::M,
                    jump: JumpCondition::Never,
                })]
            )
            .into_iter(),
            Arithmetic::Sub => const_concat!(
                STACK_POP,
                DEREF_TO_D,
                DECREMENT_POINTER,
                [Item::Instruction(Instruction::C {
                    expr: CExpr::XMinusD(Source::Memory),
                    dst: Dst::M,
                    jump: JumpCondition::Never,
                })]
            ),
            Arithmetic::Neg => todo!(),
            Arithmetic::Eq => todo!(),
            Arithmetic::Gt => todo!(),
            Arithmetic::Lt => todo!(),
            Arithmetic::And => todo!(),
            Arithmetic::Or => todo!(),
            Arithmetic::Not => todo!(),
        }
    }
}
