use crate::parse::cinstr::CTriple;
use crate::parse::space::line_spaced;
use crate::parse::{Ident, Instruction, PResult, Program};
use nom::branch::alt;
use nom::character::complete::{alphanumeric1, digit1};
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::preceded;
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
    if instruction_line.chars().nth(0) == Some('@') {
        a_instruction(instruction_line)
    } else {
        c_instruction(instruction_line)
    }
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
        Err(_) => alphanumeric1(ident).map(|(x, sym)| (x, Ident::Name(sym.to_string()))),
    }
}
