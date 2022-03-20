use crate::parse::{CExpr, Dst, Ident, JumpCondition};

pub const fn cinstr(expr: &CExpr, dst: &Dst, jmp: &JumpCondition) -> u16 {
    0b1110_0000_0000_0000 | expr.as_bits() | dst.as_bits() | jmp.as_bits()
}
