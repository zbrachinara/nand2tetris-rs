use crate::const_concat;
use crate::translate::common::*;
use n2t_asm::parse::{CExpr, Dst, Ident, Instruction, Item, JumpCondition, Source};
use std::iter::once;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn table_ptr_access(table_ptr_addr: u16, offset: u16, push_or_pop: &Stack) -> Vec<Item> {
    if *push_or_pop == Stack::Push {
        const_concat!(
            [Item::Instruction(Instruction::A(Ident::Addr(
                table_ptr_addr
            )))],
            DEREF_TO_D,
            [
                Item::Instruction(Instruction::A(Ident::Addr(offset))),
                Item::Instruction(Instruction::C {
                    expr: CExpr::DPlusX(Source::Register),
                    dst: Dst::A,
                    jump: JumpCondition::Never,
                })
            ],
            DEREF_TO_D,
            FETCH_STACK_POINTER,
            DEREF_TO_A,
            WRITE_FROM_D,
        )
        .into_iter()
        .collect()
    } else {
        todo!()
    }
}

impl Stack {
    pub fn translate<'a>(&self, mut words: impl Iterator<Item = &'a str>) -> Result<Vec<Item>, ()> {
        if let Some(Ok(segment)) = words.next().map(|seg_ident| Segment::from_str(seg_ident)) {
            let offset = words
                .next()
                .map(|s| u16::from_str_radix(s, 10).map_err(|_| ()))
                .unwrap_or(Err(()))?;
            segment.translate(offset, self)
        } else {
            Err(())
        }
    }
}

impl Segment {
    pub fn translate(&self, offset: u16, push_or_pop: &Stack) -> Result<Vec<Item>, ()> {
        match self {
            Segment::Local => Ok(table_ptr_access(1, offset, push_or_pop)),
            Segment::Argument => todo!(),
            Segment::This => todo!(),
            Segment::That => todo!(),
            Segment::Pointer => todo!(),
            Segment::Temp => todo!(),
            Segment::Static => todo!(),
            Segment::Constant => todo!(),
        }
        // todo!()
    }
}

#[derive(EnumString, Debug, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Stack {
    Push,
    Pop,
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Pointer,
    Temp,
    Static,
    Constant,
}
