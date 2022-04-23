macro_rules! n2tasm {
    ($($instr:tt)*) => {
        [$(_n2tasm_one!($instr)),*]
    };
}

macro_rules! _n2tasm_one {
    // labels
    ({(s:$lb:ident)}) => {
        println!(r#"a label with context-derived name {}"#, $lb)
    };
    ({($lb:ident)}) => {
        println!(r#"label with literal name {}"#, stringify!($lb))
    };

    // A-instruction
    ({@$ident:expr}) => {
        println!(r#"A-instruction with value "{}""#, stringify!($ident))
    };

    // C-instruction
    ({$expr:expr;$jmp:ident}) => {
        println!(r#"expression: "{}", jump command: "{}""#, stringify!($expr), stringify!($jmp))
    };
    ({$expr:expr}) => {
        println!(r#"expression: "{}" (without jump)"#, stringify!($expr))
    };
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
