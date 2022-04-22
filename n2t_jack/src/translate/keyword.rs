use strum_macros::EnumString;

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Arithmetic {
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

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Memory {
    Push,
    Pop,
}
