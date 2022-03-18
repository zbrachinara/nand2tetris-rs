use bitflags::bitflags;
use derive_more::Deref;
use strum_macros::EnumString;

#[derive(Deref, Debug)]
pub struct Program(pub Vec<Instruction>);

#[derive(Debug)]
pub enum Instruction {
    A(Ident),
    C {
        expr: CExpr,
        dst: Dst,
        jump: JumpCondition,
    },
}

#[derive(Debug)]
pub enum Ident {
    Name(String),
    Addr(u16),
}

#[derive(Debug)]
pub enum CExpr {
    Zero,
    One,
    NegOne,
    MinusOne,
    D,
    X(Source),
    NotD,
    NotX(Source),
    NegD,
    NegX(Source),
    DPlusOne,
    DMinusOne,
    XPlusOne(Source),
    XMinusOne(Source),
    DPlusX(Source),
    DMinusX(Source),
    XMinusD(Source),
    DAndX(Source),
    DOrX(Source),
}

#[derive(Debug)]
pub enum Source {
    Register,
    Memory,
}

bitflags! {
    pub struct Dst: u8 {
        const M = 0b001;
        const D = 0b010;
        const A = 0b100;
    }
}

#[derive(EnumString, Debug)]
pub enum JumpCondition {
    #[strum(disabled)]
    Never,
    #[strum(serialize = "jmp")]
    Always,
    #[strum(serialize = "jgt")]
    GreaterThan,
    #[strum(serialize = "jlt")]
    LessThan,
    #[strum(serialize = "jge")]
    GreaterEqual,
    #[strum(serialize = "jle")]
    LessEqual,
    #[strum(serialize = "jeq")]
    Equal,
    #[strum(serialize = "jne")]
    NEqual,
}
