use crate::parse::{CExpr, Dst, JumpCondition};

const fn convert_cinstr(expr: CExpr, dst: Dst, jmp: JumpCondition) -> u16 {
    0b1000_0000_0000_0000 | expr.as_bits() | dst.as_bits() | jmp.as_bits()
}
