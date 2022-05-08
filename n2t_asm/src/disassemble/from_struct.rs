use crate::parse::Program;

pub fn from_struct(s: Program) -> impl Iterator<Item = String> {
    s.0.into_iter().map(|item| {

        todo!()
    })
}
