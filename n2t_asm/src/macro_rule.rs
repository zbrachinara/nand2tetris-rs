#[macro_export]
macro_rules! n2tasm {
    ($($instr:tt)*) => {{
        use $crate::parse::structs::*;
        [$($crate::_n2tasm_one!($instr)),*]
    }};
}

#[macro_export]
macro_rules! _n2tasm_one {
    // labels
    ({($lb:ident)}) => {{
        let lb = stringify!($lb);
        $crate::_n2tasm_one!({ (s: lb) })
    }};
    ({(s:$lb:ident)}) => {
        Item::Label($lb.to_string())
    };

    // A-instruction
    ({@n:$ident:path}) => {
        Item::Instruction(Instruction::A(Ident::Addr($ident)))
    };
    ({@$ident:expr}) => {
        Item::Instruction(Instruction::A($crate::_n2tasm_a_instr_ident!($ident)))
    };

    // C-instruction
    ({$dst:ident=$expr:tt;$jmp:ident}) => {
        Item::Instruction(Instruction::C {
            dst: $crate::_n2tasm_c_instr_dst!($dst),
            expr: $crate::_n2tasm_c_instr_expr!($expr),
            jump: $crate::_n2tasm_c_instr_jmp!($jmp),
        })
    };
    ({$expr:tt;$jmp:ident}) => {
        Item::Instruction(Instruction::C {
            dst: Dst::empty(),
            expr: $crate::_n2tasm_c_instr_expr!($expr),
            jump: $crate::_n2tasm_c_instr_jmp!($jmp),
        })
    };
    ({$dst:ident=$expr:tt$(;)?}) => {
        Item::Instruction(Instruction::C {
            dst: $crate::_n2tasm_c_instr_dst!($dst),
            expr: $crate::_n2tasm_c_instr_expr!($expr),
            jump: JumpCondition::Never,
        })
    };
    ({$expr:tt$(;)?}) => {
        Item::Instruction(Instruction::C {
            dst: Dst::empty(),
            expr: $crate::_n2tasm_c_instr_expr!($expr),
            jump: JumpCondition::Never,
        })
    };
}

#[macro_export]
macro_rules! _n2tasm_a_instr_ident {
    ($id:ident) => {
        Ident::Name(stringify!($id))
    };
    ($id:literal) => {
        Ident::Addr($id)
    };
}

#[macro_export]
macro_rules! _n2tasm_c_instr_dst {
    (A) => {
        Dst::A
    };
    (M) => {
        Dst::M
    };
    (D) => {
        Dst::D
    };
    (AM) => {
        Dst::A | Dst::M
    };
    (MA) => {
        Dst::A | Dst::M
    };
    (AD) => {
        Dst::A | Dst::D
    };
    (DA) => {
        Dst::A | Dst::D
    };
    (MD) => {
        Dst::M | Dst::D
    };
    (DM) => {
        Dst::M | Dst::D
    };
    (AMD) => {
        Dst::A | Dst::M | Dst::D
    };
    (ADM) => {
        Dst::A | Dst::M | Dst::D
    };
    (MAD) => {
        Dst::A | Dst::M | Dst::D
    };
    (MDA) => {
        Dst::A | Dst::M | Dst::D
    };
    (DAM) => {
        Dst::A | Dst::M | Dst::D
    };
    (DMA) => {
        Dst::A | Dst::M | Dst::D
    };
}

#[macro_export]
macro_rules! _n2tasm_c_instr_jmp {
    (JMP) => {
        JumpCondition::Always
    };
    (JGT) => {
        JumpCondition::GreaterThan
    };
    (JLT) => {
        JumpCondition::LessThan
    };
    (JGE) => {
        JumpCondition::GreaterEqual
    };
    (JLE) => {
        JumpCondition::LessEqual
    };
    (JEQ) => {
        JumpCondition::Equal
    };
    (JNE) => {
        JumpCondition::NEqual
    };
}

#[macro_export]
macro_rules! _n2tasm_c_instr_expr {
    ((0)) => {
        CExpr::Zero
    };
    ((1)) => {
        CExpr::One
    };
    ((-1)) => {
        CExpr::NegOne
    };

    ((D)) => {
        CExpr::D
    };
    ((A)) => {
        CExpr::X(Source::Register)
    };
    ((M)) => {
        CExpr::X(Source::Memory)
    };

    ((!D)) => {
        CExpr::NotD
    };
    ((!A)) => {
        CExpr::NotX(Source::Register)
    };
    ((!M)) => {
        CExpr::NotX(Source::Memory)
    };

    ((-D)) => {
        CExpr::NegD
    };
    ((-A)) => {
        CExpr::NegX(Source::Register)
    };
    ((-M)) => {
        CExpr::NegX(Source::Memory)
    };

    ((D+1)) => {
        CExpr::DPlusOne
    };
    ((A+1)) => {
        CExpr::XPlusOne(Source::Register)
    };
    ((M+1)) => {
        CExpr::XPlusOne(Source::Memory)
    };

    ((D-1)) => {
        CExpr::DMinusOne
    };
    ((A-1)) => {
        CExpr::XMinusOne(Source::Register)
    };
    ((M-1)) => {
        CExpr::XMinusOne(Source::Memory)
    };

    ((D+A)) => {
        CExpr::DPlusX(Source::Register)
    };
    ((A+D)) => {
        CExpr::DPlusX(Source::Register)
    };
    ((D+M)) => {
        CExpr::DPlusX(Source::Memory)
    };
    ((M+D)) => {
        CExpr::DPlusX(Source::Memory)
    };

    ((D-A)) => {
        CExpr::DMinusX(Source::Register)
    };
    ((D-M)) => {
        CExpr::DMinusX(Source::Memory)
    };
    ((A-D)) => {
        CExpr::XMinusD(Source::Register)
    };
    ((M-D)) => {
        CExpr::XMinusD(Source::Memory)
    };

    ((D&A)) => {
        CExpr::DAndX(Source::Register)
    };
    ((A&D)) => {
        CExpr::DAndX(Source::Register)
    };
    ((D&M)) => {
        CExpr::DAndX(Source::Memory)
    };
    ((M&D)) => {
        CExpr::DAndX(Source::Memory)
    };

    ((D|A)) => {
        CExpr::DOrX(Source::Register)
    };
    ((A|D)) => {
        CExpr::DOrX(Source::Register)
    };
    ((D|M)) => {
        CExpr::DOrX(Source::Memory)
    };
    ((M|D)) => {
        CExpr::DOrX(Source::Memory)
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_macro() {
        let label_name = "label_test";
        let location = 2;

        n2tasm! {
            {(s:label_name)}
            {(abcdef)}
            {@0}
            {@n:location}
            {M=(M+1);JEQ}
            {ADM=(M);}
        };
        // _n2tasm_one!({ (-1) });
        // _n2tasm_one!({ (M + 1) });
    }
}
