use crate::parse::cinstr::CTriple;
use crate::parse::space::{line_spaced, spaced};
use crate::parse::{Ident, Instruction, PResult, Program};
use nom::branch::alt;
use nom::character::complete::{alphanumeric1, digit1};
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use nom_supreme::tag::complete::tag;
use std::str::FromStr;

pub fn program(program: &str) -> PResult<Program> {
    many1(line_spaced(instruction))
        .map(|vec| Program(vec))
        .parse(program)
}

// instruction line must begin on the first character of the instruction
fn instruction(instruction_line: &str) -> PResult<Instruction> {
    match instruction_line.chars().nth(0) {
        Some('@') => a_instruction(instruction_line),
        Some('(') => label(instruction_line),
        _ => c_instruction(instruction_line),
    }
}

fn label(lb: &str) -> PResult<Instruction> {
    delimited(spaced(tag("(")), identifier_name_only, spaced(tag(")")))
        .map(|lb_str| Instruction::Label(lb_str))
        .parse(lb)
}

// in an a-instruction, the @ and identifier must not be separated by any kind of space
fn a_instruction(instruction: &str) -> PResult<Instruction> {
    preceded(tag("@"), identifier)
        .map(|ident| Instruction::A(ident))
        .parse(instruction)
}

fn c_instruction(instruction: &str) -> PResult<Instruction> {
    CTriple::from_string(instruction).and_then(|(x, triple)| {
        // TODO: change weird error message
        triple
            .to_cinstr()
            .map_err(|_| {
                nom::Err::Error(nom::error::Error {
                    input: "",
                    code: ErrorKind::Many0,
                })
            })
            .map(|triple| (x, triple))
    })
}

fn identifier(ident: &str) -> PResult<Ident> {
    match digit1::<_, nom::error::Error<_>>(ident) {
        // numeric constant
        Ok((x, c)) => Ok((x, Ident::Addr(u16::from_str(c).unwrap()))),
        // symbol
        Err(_) => many1(alt((alphanumeric1, tag("_"))))
            .map(|v| v.join(""))
            .map(|sym| Ident::Name(sym.to_string()))
            .parse(ident),
        // Err(_) => alphanumeric1(ident).map(|(x, sym)| (x, Ident::Name(sym.to_string()))),
    }
}

fn identifier_name_only(ident: &str) -> PResult<Ident> {
    identifier(ident).and_then(|(x, id)| {
        match id {
            Ident::Name(_) => Ok((x, id)),
            Ident::Addr(_) => Err(nom::Err::Error(nom::error::Error {
                // TODO: Fix this weird error message too
                input: "",
                code: ErrorKind::Tag,
            }))
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        assert!(matches!(identifier("MAIN_LOOP"), Ok((_, Ident::Name(x))) if x == "MAIN_LOOP"));

        label("(WHEN_DEEZ)").unwrap();
    }

    #[test]
    fn theoretical() {
        let single_instruction = program(
            r#"

        DM=M+D

        "#,
        )
        .unwrap()
        .1;

        println!("{single_instruction:#?}");

        let a_c_instruction = program(
            r#"

        @42069
        DM=M+D

        "#,
        )
        .unwrap()
        .1;

        println!("{a_c_instruction:#?}");
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
        .1;

        println!("{mult:#?}")
    }
}
