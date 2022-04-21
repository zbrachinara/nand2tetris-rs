use crate::err::AssemblyError;
use nom::IResult;
use nom_locate::LocatedSpan;
use crate::assemble::{Address, SymbolTable};

mod space;
mod cinstr;
mod parsing;
mod util;
pub mod structs;

pub use structs::*;

pub type Span<'a> = LocatedSpan<&'a str>;
type PResult<'a, I> = IResult<Span<'a>, I, AssemblyError>;

pub fn program(program: &str) -> Result<(Program, SymbolTable), AssemblyError> {
    let mut sym_table = SymbolTable::new();

    let mut line = 0;

    let program = parsing::program(program.into())
        .map(|res| res.map_err(nom::Err::into).map(|(_, p)| p))
        .filter_map(|item| match item {
            Ok(Item::Label(lb)) => {
                sym_table.insert(lb, Address::Rom(line));
                None
            }
            Ok(Item::Instruction(x)) => {
                line += 1;
                Some(Ok(x))
            }
            Err(x) => Some(Err(x)),
        })
        .try_collect::<Vec<_>>()
        .map(Program);

    program.map(|p| (p, sym_table))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn theoretical() {
        let (single_instruction, _) = program(
            r#"

        DM=M+D

        "#,
        )
            .unwrap();

        println!("{single_instruction:#?}");

        let (a_c_instruction, _) = program(
            r#"

        @42069
        DM=M+D

        "#,
        )
            .unwrap();

        println!("{a_c_instruction:#?}");
    }

    #[test]
    fn problems() {
        println!("{:?}", program("D;JGT\n").unwrap().0);
    }

    #[test]
    fn practical() {
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
            .unwrap()
            .0;

        println!("{mult:#?}");
    }
}
