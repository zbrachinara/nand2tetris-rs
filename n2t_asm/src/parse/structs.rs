use bitflags::bitflags;
use strum_macros::EnumString;

#[derive(Debug)]
pub struct Program(pub Vec<Instruction>);

#[derive(Debug)]
pub enum Item {
    Label(String),
    Instruction(Instruction),
}

#[derive(Debug)]
pub enum Instruction {
    A(Ident),
    // Label(String),
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
    #[strum(serialize = "JMP")]
    Always,
    #[strum(serialize = "JGT")]
    GreaterThan,
    #[strum(serialize = "JLT")]
    LessThan,
    #[strum(serialize = "JGE")]
    GreaterEqual,
    #[strum(serialize = "JLE")]
    LessEqual,
    #[strum(serialize = "JEQ")]
    Equal,
    #[strum(serialize = "JNE")]
    NEqual,
}
