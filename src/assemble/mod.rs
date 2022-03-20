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
    program
        .iter()
        .filter_map(|instr| {
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
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::program;

    #[test]
    fn mult() {
        let mult = program(
            r#"
@R2
M=0

(MULT_LOOP)

// exit if R0 == 0
@R0
D=M
@EXIT
D;JEQ

// increase R2 by R1
@R1
D=M
@R2
M=M+D

// decrement R0
@R0
M=M-1

// loop
@MULT_LOOP
0;JMP

(EXIT)
@EXIT
0;JMP
        "#,
        )
        .unwrap();

        let mult_code = assemble_program(mult);

        let compare = &[
            0b0000000000000010,
            0b1110101010001000,
            0b0000000000000000,
            0b1111110000010000,
            0b0000000000001110,
            0b1110001100000010,
            0b0000000000000001,
            0b1111110000010000,
            0b0000000000000010,
            0b1111000010001000,
            0b0000000000000000,
            0b1111110010001000,
            0b0000000000000010,
            0b1110101010000111,
            0b0000000000001110,
            0b1110101010000111,
        ];

        mult_code.iter().enumerate().for_each(|(i, n)| {
            assert_eq!(&compare[i], n, "assertion failed on instruction {i}");
            println!("{n:#b}")
        });
    }
}
