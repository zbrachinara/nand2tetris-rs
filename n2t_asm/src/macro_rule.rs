#[macro_export]
macro_rules! n2tasm {
    ($($instr:tt)*) => {
        [$($crate::_n2tasm_one!($instr)),*]
    };
}

#[macro_export]
macro_rules! _n2tasm_one {
    // labels
    ({($lb:ident)}) => {{
        print!(r#"label with literal name: "#);
        let lb = stringify!($lb);
        $crate::_n2tasm_one!({(s:lb)})
    }};
    ({(s:$lb:ident)}) => {{
        use $crate::parse::structs::Item;
        println!(r#"label {}"#, $lb);
        Item::Label($lb.to_string())
    }};

    // A-instruction
    ({@$ident:expr}) => {{
        use $crate::parse::structs::{Item, Instruction};
        println!(r#"A-instruction with value "{}""#, stringify!($ident));
        Item::Instruction(Instruction::A(_n2tasm_a_instr_ident!($ident)))
    }};

    // C-instruction
    ({$expr:expr;$jmp:ident}) => {{
        println!(r#"expression: "{}", jump command: "{}""#, stringify!($expr), stringify!($jmp));
        todo!()
    }};
    ({$expr:expr}) => {{
        println!(r#"expression: "{}" (without jump)"#, stringify!($expr));
        todo!()
    }};
}

macro_rules! _n2tasm_a_instr_ident {
    ($id:ident) => {{
        use $crate::parse::structs::Ident;
        Ident::Name(stringify!($id))
    }};
    ($id:literal) => {{
        use $crate::parse::structs::Ident;
        Ident::Addr($id)
    }}
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_macro() {
        let label_name = "label_test";

        n2tasm! {
            {(s:label_name)}
            {(abcdef)}
            {@0}
            {D=M}
            {M=M+1;JEQ}
        };
        _n2tasm_one!({-1});
        _n2tasm_one!({M+1});
    }
}
