mod predefined;

use crate::assemble::predefined::new_symbol_table;
use crate::parse::{Instruction, Program};

pub fn assemble_program(program: Program) -> Vec<u16> {
    let output = vec![];

    let Program(program) = program;
    let (labels, instructions) = program
        .into_iter()
        .partition::<Vec<_>, _>(|instr| matches!(instr, Instruction::Label(_)));

    let symbol_table = new_symbol_table();

    for instr in instructions {

    }


    output
}
