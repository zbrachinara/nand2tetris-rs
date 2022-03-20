mod predefined;

use crate::assemble::predefined::SymbolTable;
use crate::parse::{Instruction, Program};

pub fn assemble_program(program: Program) -> Vec<u16> {
    let output = vec![];

    let Program(program) = program;
    let (labels, instructions) = program
        .into_iter()
        .partition::<Vec<_>, _>(|instr| matches!(instr, Instruction::Label(_)));

    let symbol_table = SymbolTable::new();

    for instr in instructions {}

    output
}
