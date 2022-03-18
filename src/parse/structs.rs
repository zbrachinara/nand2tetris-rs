use bitflags::bitflags;
use derive_more::Deref;

#[derive(Deref)]
pub struct Program(pub Vec<Instruction>);

pub enum Instruction {
    A(Ident),
    C {
        expr: CExpr,
        dst: Dst,
        jump: JumpCondition,
    },
}

pub enum Ident {
    Name(String),
    Addr(u16),
}

pub enum CExpr {
    Zero,
    One,
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

pub enum JumpCondition {
    Never,
    Always,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
    NEqual,
}
