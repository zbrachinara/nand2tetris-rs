use n2t_asm::parse::Item;
use std::str::FromStr;
use strum_macros::EnumString;

impl Stack {
    pub fn translate<'a>(&self, mut words: impl Iterator<Item = &'a str>) -> Result<Vec<Item>, ()> {
        if let Some(Ok(segment)) = words.next().map(|seg_ident| Segment::from_str(seg_ident)) {
            let offset = words
                .next()
                .map(|s| u16::from_str_radix(s, 10).map_err(|_| ()))
                .unwrap_or(Err(()))?;
            segment.translate(offset)
        } else {
            Err(())
        }
    }
}

impl Segment {
    pub fn translate(&self, offset: u16) -> Result<Vec<Item>, ()> {
        todo!()
    }
}

#[derive(EnumString, Debug)]
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
