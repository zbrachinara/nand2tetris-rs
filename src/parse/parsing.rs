use crate::err::AssemblyError;
use crate::parse::cinstr::CTriple;
use crate::parse::space::{line_spaced, spaced};
use crate::parse::{Ident, Instruction, Item, PResult};
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::Parser;
use std::str::FromStr;

pub fn program(program: &str) -> impl Iterator<Item = PResult<Item>> {
    program
        .split('\n')
        .filter(|s| s.trim_start() != "" && !s.trim_start().starts_with("//"))
        .map(|s| line_spaced(instruction).parse(s))
}

// instruction line must begin on the first character of the instruction
fn instruction(instruction_line: &str) -> PResult<Item> {
    #[cfg(debug_assertions)]
    match instruction_line.bytes().next() {
        Some(b'@') => crate::time!(a_instruction(instruction_line)),
        Some(b'(') => crate::time!(label(instruction_line)),
        Some(_) => crate::time!(c_instruction(instruction_line)),
        _ => unreachable!("instruction line must begin with @, (, or a letter"),
    }
    #[cfg(not(debug_assertions))]
    match instruction_line.bytes().next() {
        Some(b'@') => a_instruction(instruction_line),
        Some(b'(') => label(instruction_line),
        Some(_) => c_instruction(instruction_line),
        _ => unreachable!("instruction line must begin with @, (, or a letter"),
    }
}

fn label(lb: &str) -> PResult<Item> {
    delimited(spaced(char('(')), identifier_name_only, spaced(char(')')))
        .map(Item::Label)
        .parse(lb)
}

// in an a-instruction, the @ and identifier must not be separated by any kind of space
fn a_instruction(instruction: &str) -> PResult<Item> {
    preceded(char('@'), identifier)
        .map(|x| Item::Instruction(Instruction::A(x)))
        .parse(instruction)
}

fn c_instruction(instruction: &str) -> PResult<Item> {
    CTriple::from_string(instruction).and_then(|(x, triple)| {
        triple
            .to_cinstr()
            .map_err(|_| nom::Err::Error(AssemblyError::InvalidCExpr))
            .map(Item::Instruction)
            .map(|triple| (x, triple))
    })
}

fn identifier(ident: &str) -> PResult<Ident> {
    match digit1::<_, nom::error::Error<_>>(ident) {
        // numeric constant
        Ok((x, c)) => Ok((x, Ident::Addr(u16::from_str(c).unwrap()))),
        // symbol
        Err(_) => many1(alt((alphanumeric1, is_a("_.$"))))
            .map(|v| v.join(""))
            .map(Ident::Name)
            .parse(ident),
    }
}

fn identifier_name_only(ident: &str) -> PResult<String> {
    identifier(ident).and_then(|(x, id)| match id {
        Ident::Name(id) => Ok((x, id)),
        Ident::Addr(_) => Err(nom::Err::Error(AssemblyError::InvalidIdentifier)),
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
}
