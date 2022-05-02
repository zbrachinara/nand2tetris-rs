use crate::const_concat;
use crate::translate::common::*;
use n2t_asm::n2tasm;
use n2t_asm::parse::{CExpr, Dst, Ident, Instruction, Item, JumpCondition, Source};
use std::str::FromStr;
use strum_macros::EnumString;

pub fn table_access(table_addr: u16, offset: u16, push_or_pop: &Stack) -> Vec<Item> {
    match push_or_pop {
        Stack::Push => const_concat!(
            [
                Item::Instruction(Instruction::A(Ident::Addr(table_addr))),
                Item::Instruction(Instruction::C {
                    expr: CExpr::X(Source::Register),
                    dst: Dst::D,
                    jump: JumpCondition::Never,
                }),
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
        .collect::<Vec<_>>(),
        Stack::Pop => {
            todo!()
        }
    }
}

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

fn tabled_segment(table_offset: u16, segment_offset: u16, push_or_pop: &Stack) -> Vec<Item> {
    match push_or_pop {
        Stack::Push => {
            n2tasm!(
                {@n:table_offset}   // get the base ptr to the table
                {D=(M)}             // store base pointer to D
                {@n:segment_offset} // get the offset from the base ptr
                {A=(D+A)}           // add the offset to the base ptr
                {D=(M)}             // store the value at the offset to D

                {@0}
                {M=(M+1)}           // increment the stack ptr
                {A=(M-1)}           // get the empty stack location

                {M=(D)}             // store the value at the offset to the empty address
            )
            .to_vec()
        }
        Stack::Pop => {
            n2tasm!(
                {@n:table_offset}
                {D=(M)}
                {@n:segment_offset}
                {D=(D+A)}           // calculate destination ptr and store it in D

                {@0}
                {M=(M-1)}           // decrement the stack ptr
                {A=(M+1)}           // go one above the end of the stack
                {M=(D)}             // store destination ptr above the stack

                {A=(A-1)}
                {D=(M)}             // load stack head
                {A=(A+1)}
                {A=(M)}             // load destination ptr

                {M=(D)}             // perform popping operation
            ).to_vec()
        }
    }
}

fn push_const(value: u16) -> Vec<Item> {
    n2tasm!(
        {@n:value}
        {D=(A)}

        {@0}
        {M=(M+1)}
        {A=(M-1)}
        {M=(D)}
    )
    .to_vec()
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
            Segment::Local => Ok(tabled_segment(1, offset, push_or_pop)),
            Segment::Argument => Ok(tabled_segment(2, offset, push_or_pop)),
            Segment::This => Ok(tabled_segment(3, offset, push_or_pop)),
            Segment::That => Ok(tabled_segment(4, offset, push_or_pop)),
            Segment::Constant => match push_or_pop {
                Stack::Push => Ok(push_const(offset)),
                Stack::Pop => Err(()),
            },
            Segment::Pointer => match offset {
                0 => todo!(),
                1 => todo!(),
                _ => Err(()),
            },
            Segment::Temp => todo!(),
            Segment::Static => todo!(),
        }
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
