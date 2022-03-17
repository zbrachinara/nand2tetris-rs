use crate::parse::space::spaced;
use crate::parse::{Ident, Instruction, Program};
use nom::multi::many1;
use nom::{IResult, Parser};

type PResult<'a, T> = IResult<&'a str, T>;

pub fn program(program: &str) -> PResult<Program> {
    many1(spaced(instruction))
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

fn a_instruction(instruction: &str) -> PResult<Instruction> {
    todo!()
}

fn c_instruction(instruction: &str) -> PResult<Instruction> {
    todo!()
}

fn identifier(ident: &str) -> PResult<Ident> {
    todo!()
}
