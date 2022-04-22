use n2t_asm::parse::Item;
use strum_macros::EnumString;

impl Stack {
    pub fn translate(&self, words: impl Iterator<Item = String>) -> Vec<Item> {

        todo!()
    }
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Stack {
    Push,
    Pop,
}
