use crate::parse::{Instruction, Item, Ident};

pub fn from_struct(s: impl IntoIterator<Item = Item>) -> impl Iterator<Item = String> {
    s.into_iter().map(|item| match item {
        Item::Label(lb) => {
            format!("({lb})")
        }
        Item::Instruction(Instruction::A(Ident::Addr(x))) => {
            format!("@{x}")           
        }
        Item::Instruction(Instruction::A(Ident::Name(s))) => {
            format!("@{s}")
        }
        Item::Instruction(Instruction::C { expr: _, dst: _, jump: _ }) => {
            todo!()
        }
    })
}
