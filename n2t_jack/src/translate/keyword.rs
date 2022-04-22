use strum_macros::EnumString;

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Memory {
    Push,
    Pop,
}
