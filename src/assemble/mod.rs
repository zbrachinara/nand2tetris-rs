mod convert;
mod predefined;
mod symbol_table;

use crate::parse::{Ident, Instruction, Program};
use symbol_table::{Address, SymbolTable};

pub fn assemble_program(program: Program) -> Vec<u16> {
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

    // final processing
    program.iter().filter_map(|instr| {
        match instr {
            Instruction::A(ident) => Some(
                0b0111_1111_1111_1111
                    & match ident {
                        Ident::Name(str) => symbol_table[str].unwrap(),
                        Ident::Addr(addr) => *addr,
                    },
            ),
            Instruction::C { expr, dst, jump } => Some(convert::cinstr(&expr, &dst, &jump)),
            _ => None, // ignore
        }
    }).collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn practical() {

    }
}