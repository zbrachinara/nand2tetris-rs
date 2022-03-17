use nom::IResult;
use crate::parse::{Ident, Instruction, Program};

type PResult<'a, T> = IResult<&'a str, T>;

pub fn program(program: &str) -> PResult<Program> {
    todo!()
}

fn instruction(instruction_line: &str) -> PResult<Instruction> {
    todo!()
}

fn a_instruction(instruction: &str) -> PResult<Instruction> {
    todo!()
}

fn c_instruction(instruction: &str) -> PResult<Instruction> {
    todo!()
}

fn identifier(ident: &str) -> PResult<Ident> {
    todo!()
}

