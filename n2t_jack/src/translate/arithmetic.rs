use super::common::*;
use crate::const_concat;
use n2t_asm::parse::{CExpr, Dst, Instruction, Item, JumpCondition};
use n2t_asm::parse::{Ident, Source};
use n2t_asm::n2tasm;
use strum_macros::EnumString;

const HIGH_BIT: u16 = 0b1000_0000_0000_0000;

const ADD: &[Item] = &n2tasm! {
    {@0}      //Addressing stack pointer
    {M=(M-1)} // pop stack
    {A=(M)}
    {D=(M)}   // retrieve popped stack value
    {A=(A-1)}
    {M=(D+M)} // add popped value and stack end value
};

const SUB: &[Item] = &n2tasm! {
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(D-M)}
};

const NEG: &[Item] = &n2tasm! {
    {@0}
    {A=(M-1)} // addressing stack end value
    {M=(-M)}
};

const AND: &[Item] = &n2tasm! {
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(D&M)}
};

const OR: &[Item] = &n2tasm!(
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(D|M)}
);

const NOT: &[Item] = &n2tasm!(
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(!M)}
);

const EQ: &[Item] = &[]; // TODO: Previous EQ implementation was completely broken -- fix

const LT: &[Item] = &n2tasm!{
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(M-D)} // high bit of M is now set if M < D
    {@0b1000_0000_0000_0000}
    {D=(A)}
    {@0}
    {A=(M-1)}
    {M=(D&M)}
};

const GT: &[Item] = &n2tasm!{
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(D-M)} // high bit of M is now set if D < M
    {@0b1000_0000_0000_0000}
    {D=(A)}
    {@0}
    {A=(M-1)}
    {M=(D&M)}
};

impl Arithmetic {
    pub fn translate(self) -> &'static [Item] {
        match self {
            Arithmetic::Add => ADD,
            Arithmetic::Sub => SUB,
            Arithmetic::Neg => NEG,
            Arithmetic::Eq => todo!(),
            Arithmetic::Gt => GT,
            Arithmetic::Lt => LT,
            Arithmetic::And => AND,
            Arithmetic::Or => OR,
            Arithmetic::Not => NOT,
        }
    }
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    /// ```text
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// M=M-D
    /// M=!M
    /// ```
    Eq,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=D-M
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Gt,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=M-D
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Lt,
    And,
    Or,
    Not,
}
