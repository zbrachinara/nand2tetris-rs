mod predefined;

use crate::assemble::predefined::{Address, SymbolTable};
use crate::parse::{Instruction, Program};

pub fn assemble_program(program: Program) -> Vec<u16> {
    let output = vec![];
    let mut symbol_table = SymbolTable::new();

    // populate symbol table with rom addresses
    {
        let mut instr_count = 0;
        for instr in program {
            match instr {
                Instruction::Label(str) => {
                    symbol_table.insert(str.clone(), Address::Rom(instr_count))
                }
                _ => instr_count += 1,
            }
        }
    }

    let Program(program) = program;
    let (_, instructions) = program
        .into_iter()
        .partition::<Vec<_>, _>(|instr| matches!(instr, Instruction::Label(_)));

    for instr in instructions {}

    output
}
