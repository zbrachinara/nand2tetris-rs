use n2t_asm::n2tasm;
use n2t_asm::parse::Item;
use std::str::FromStr;
use strum_macros::EnumString;

fn segment_table_addr(table_offset: u16, segment_offset: u16, push_or_pop: &Stack) -> Vec<Item> {
    match push_or_pop {
        Stack::Push => n2tasm!(
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
        .to_vec(),
        Stack::Pop => n2tasm!(
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
        )
        .to_vec(),
    }
}

fn segment_static_addr(addr: u16, segment_offset: u16, push_or_pop: &Stack) -> Vec<Item> {
    let destination = addr + segment_offset;
    match push_or_pop {
        Stack::Push => n2tasm!(
            {@n:destination}
            {D=(M)}

            {@0}
            {M=(M+1)}
            {A=(M-1)}

            {M=(D)}
        )
        .to_vec(),
        Stack::Pop => n2tasm!(
            {@0}
            {M=(M-1)}
            {A=(M)}
            {D=(M)}             // perform pop

            {@n:destination}
            {M=(D)}             // load value to destination
        )
        .to_vec(),
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
            Segment::Local => Ok(segment_table_addr(1, offset, push_or_pop)),
            Segment::Argument => Ok(segment_table_addr(2, offset, push_or_pop)),
            Segment::This => Ok(segment_table_addr(3, offset, push_or_pop)),
            Segment::That => Ok(segment_table_addr(4, offset, push_or_pop)),
            Segment::Constant => match push_or_pop {
                Stack::Push => Ok(push_const(offset)),
                Stack::Pop => Err(()),
            },
            Segment::Static => Ok(segment_static_addr(16, offset, push_or_pop)),
            Segment::Temp => Ok(segment_static_addr(5, offset, push_or_pop)),
            Segment::Pointer => match offset {
                0 => todo!(),
                1 => todo!(),
                _ => Err(()),
            },
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
