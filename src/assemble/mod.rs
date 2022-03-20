mod predefined;
mod symbol_table;
mod convert;

use symbol_table::{Address, SymbolTable};
use crate::parse::{Instruction, Program};

pub fn assemble_program(program: Program) -> Vec<u16> {
    let output = vec![];
    let mut symbol_table = SymbolTable::new();

    // populate symbol table with rom addresses
    {
        let mut instr_count = 0;
        for instr in program.iter() {
            match instr {
                Instruction::Label(str) => {
                    symbol_table.insert(str.clone(), Address::Rom(instr_count))
                }
                _ => instr_count += 1,
            }
        }
    }

    for instr in program.iter() {
        match instr {
            Instruction::A(_) => todo!(),
            Instruction::C{ .. } => todo!(),
            _ => () // ignore
        }
    }

    output
}
