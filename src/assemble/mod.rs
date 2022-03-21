mod convert;
mod predefined;
mod symbol_table;

use crate::parse::{Ident, Instruction, Program};
use symbol_table::{Address, SymbolTable};

pub fn to_string(program: &Program) -> String {
    to_vec(program)
        .into_iter()
        .map(|n| format!("{n:016b}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn to_vec(program: &Program) -> Vec<u16> {
    let mut symbol_table = SymbolTable::new();

    // populate symbol table with rom addresses
    {
        let mut instr_count = 0;
        for instr in program.iter() {
            match instr {
                Instruction::Label(str) => {
                    symbol_table.insert(str.clone(), Address::Rom(instr_count));
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
                            Ident::Name(str) => symbol_table
                                .get(str.as_str())
                                .cloned()
                                .unwrap_or_else(|| {
                                    symbol_table.assign_available_ram(str.clone()).unwrap()
                                })
                                .unwrap(),
                            Ident::Addr(addr) => *addr,
                        },
                ),
                Instruction::C { expr, dst, jump } => Some(convert::cinstr(expr, dst, jump)),
                Instruction::Label(_) => None, // ignore
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

        let mult_code = to_vec(&mult);

        let compare = &[
            0b0000_0000_0000_0010,
            0b1110_1010_1000_1000,
            0b0000_0000_0000_0000,
            0b1111_1100_0001_0000,
            0b0000_0000_0000_1110,
            0b1110_0011_0000_0010,
            0b0000_0000_0000_0001,
            0b1111_1100_0001_0000,
            0b0000_0000_0000_0010,
            0b1111_0000_1000_1000,
            0b0000_0000_0000_0000,
            0b1111_1100_1000_1000,
            0b0000_0000_0000_0010,
            0b1110_1010_1000_0111,
            0b0000_0000_0000_1110,
            0b1110_1010_1000_0111,
        ];

        mult_code.iter().enumerate().for_each(|(i, n)| {
            assert_eq!(&compare[i], n, "assertion failed on instruction {i}");
        });
    }

    #[test]
    fn fill() {
        let fill = program(
            r#"

(ELOOP)

    @24576 //keyboard signal
    D=M

    @KY
    D;JGT

    @NOKY
    1; JGT

(KY)

    @8192
    D=A // initialize counter to "end" of screen buffer
    (SET)
        @SCREEN
        D=D-1 // advance to next iteration
        A=A+D // jump ptr to pixel in current iteration
        M=-1  // color current string

        // condition: The page clear has visited the last string (0)
        @ELOOP
        D;JEQ

        @SET
        0;JMP

(NOKY)

    @SCREEN
    M=0

    @8192
    D=A // initialize counter to "end" of screen buffer
    (USET)
        @SCREEN
        D=D-1 // advance to next iteration
        A=A+D // jump ptr to pixel in current iteration
        M=0  // color current string

        // condition: The page clear has visited the last string (0)
        @ELOOP
        D;JEQ

        @USET
        0;JMP

        "#,
        )
        .unwrap();

        let compare = &[
            0b0110_0000_0000_0000,
            0b1111_1100_0001_0000,
            0b0000_0000_0000_0110,
            0b1110_0011_0000_0001,
            0b0000_0000_0001_0000,
            0b1110_1111_1100_0001,
            0b0010_0000_0000_0000,
            0b1110_1100_0001_0000,
            0b0100_0000_0000_0000,
            0b1110_0011_1001_0000,
            0b1110_0000_1010_0000,
            0b1110_1110_1000_1000,
            0b0000_0000_0000_0000,
            0b1110_0011_0000_0010,
            0b0000_0000_0000_1000,
            0b1110_1010_1000_0111,
            0b0100_0000_0000_0000,
            0b1110_1010_1000_1000,
            0b0010_0000_0000_0000,
            0b1110_1100_0001_0000,
            0b0100_0000_0000_0000,
            0b1110_0011_1001_0000,
            0b1110_0000_1010_0000,
            0b1110_1010_1000_1000,
            0b0000_0000_0000_0000,
            0b1110_0011_0000_0010,
            0b0000_0000_0001_0100,
            0b1110_1010_1000_0111,
        ];

        to_vec(&fill)
            .iter()
            .enumerate()
            .for_each(|(i, n)| assert_eq!(&compare[i], n, "assertion failed on instruction {i}"));
    }
}
