use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition, Source};
use crate::const_concat;
use crate::util::const_concat;

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
///
/// The new stack pointer will be stored to A, and also points to the old stack top value
pub const STACK_POP: [Item; 3] = const_concat!(FETCH_STACK_POINTER, DECREMENT_MEM, DEREF_TO_A);

/// Sets up environment for a two-parameter op
///
/// The first parameter is stored in M, the second in A
pub const STACK_CALL_ON_TWO: [Item; 5] = const_concat!(
    STACK_POP,
    DEREF_TO_D,
    DECREMENT_POINTER,
);
