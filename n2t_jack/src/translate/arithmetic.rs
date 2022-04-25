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

const EQ: &[Item] = &n2tasm![
    {@0}
    {M=(M-1)}
    {A=(M)}
    {D=(M)}
    {A=(A-1)}
    {M=(M-D)} // high bit of M is now set if M < D
    {D=(M+D)} // D is now the initial value of arg1
    {A=(A+1)}
    {D=(M-D)} // high bit of D is now set if M < D
    {A=(A-1)}
    {M=(M|D)} // high bit set if arg1 < arg2 or arg2 < arg1
    {M=(!M)}  // high bit set if arg1 = arg2
    {@0b1000_0000_0000_0000}
    {D=(A)}
    {@0}
    {A=(M-1)} // loaded constant and returned to stack end
    {M=(D&M)} // mask out high bit
]; // TODO: Previous EQ implementation was completely broken -- fix

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
    {A=(M-1)} // loaded constant and returned to stack end
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
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}
