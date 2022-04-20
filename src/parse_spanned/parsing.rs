use crate::err::AssemblyError;
use crate::parse::{Ident, Instruction, Item};
use crate::parse_spanned::cinstr::CTriple;
use crate::parse_spanned::space::{alt_line_spaced, spaced};
use crate::parse_spanned::{PResult, Span};
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::multi::fold_many1;
use nom::sequence::{delimited, preceded};
use nom::Parser;
use std::str::FromStr;

pub fn program(program: Span) -> impl Iterator<Item = PResult<Item>> {
    super::util::many0_spliterate(alt_line_spaced(instruction), program, '\n')
        .filter_map(|res| match res {
            Ok((a, Some(b))) => Some(Ok((a, b))),
            Ok((_, None)) => None,
            Err(x) => Some(Err(x)),
        })
}

/// instruction line must begin on the first character of the instruction
fn instruction(instruction_line: Span) -> PResult<Item> {
    alt((a_instruction, label, c_instruction)).parse(instruction_line)
}

fn label(lb: Span) -> PResult<Item> {
    delimited(spaced(char('(')), identifier_name_only, spaced(char(')')))
        .map(Item::Label)
        .parse(lb)
}

// in an a-instruction, the @ and identifier must not be separated by any kind of space
fn a_instruction(instruction: Span) -> PResult<Item> {
    preceded(char('@'), identifier)
        .map(|x| Item::Instruction(Instruction::A(x)))
        .parse(instruction)
}

fn c_instruction(instruction: Span) -> PResult<Item> {
    CTriple::from_string(instruction).and_then(|(x, triple)| {
        triple
            .to_cinstr()
            .map_err(|_| nom::Err::Error(AssemblyError::InvalidCExpr))
            .map(Item::Instruction)
            .map(|triple| (x, triple))
    })
}

fn identifier(ident: Span) -> PResult<Ident> {
    match digit1::<_, nom::error::Error<_>>(ident) {
        // numeric constant
        Ok((x, c)) => Ok((x, Ident::Addr(u16::from_str(*c).unwrap()))),
        // symbol
        Err(_) => fold_many1(
            alt((alphanumeric1, is_a("_.$"))),
            || String::new(),
            |mut acc, it: Span| {
                acc.push_str(*it);
                acc
            },
        )
        .map(Ident::Name)
        .parse(ident),
    }
}

fn identifier_name_only(ident: Span) -> PResult<String> {
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
        assert!(
            matches!(identifier(Span::from("MAIN_LOOP")), Ok((_, Ident::Name(x))) if x == "MAIN_LOOP")
        );

        label(Span::from("(WHEN_DEEZ$.a)")).unwrap();
    }
}
