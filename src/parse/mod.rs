mod cinstr;
mod parsing;
mod space;
mod structs;

use crate::err::AssemblyError;
use nom::IResult;
pub use structs::*;

pub fn program(program: &str) -> Result<Program, AssemblyError> {
    if program.trim().is_empty() {
        Ok(Program(Vec::new()))
    } else {
        parsing::program(program)
            .map(|res| res.map_err(nom::Err::into).map(|(_, p)| p))
            .try_collect::<Vec<_>>()
            .map(Program)
    }
}

type PResult<'a, T> = IResult<&'a str, T, AssemblyError>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn theoretical() {
        let single_instruction = program(
            r#"

        DM=M+D

        "#,
        )
        .unwrap();

        println!("{single_instruction:#?}");

        let a_c_instruction = program(
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
        println!("{:?}", program("D;JGT\n").unwrap());
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
        .unwrap();

        println!("{mult:#?}")
    }
}
