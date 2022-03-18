use crate::parse::space::spaced;
use crate::parse::{Instruction, JumpCondition, PResult};
use nom::branch::alt;
use nom::bytes::complete::{take_until, take_while1};
use nom::character::complete::{alphanumeric1, char};
use nom::character::is_alphabetic;
use nom::combinator::opt;
use nom::sequence::{preceded, terminated, tuple};
use nom::{AsChar, Parser};
use nom_supreme::tag::complete::tag;

fn cexpr(str: &str) -> PResult<&str> {
    fn is_cexpr_char(c: char) -> bool {
        matches!(c, 'A' | 'M' | 'D' | '+' | '-' | ' ')
    }
    take_while1(is_cexpr_char).parse(str)
}

fn jexpr(str: &str) -> PResult<&str> {
    fn is_jexpr_char(c: char) -> bool {
        c.is_alphanum() || c == ' '
    }
    take_while1(is_jexpr_char)(str)
}

/// A representation of a compute instruction triple in string form
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct CTriple {
    dst: Option<String>,
    expr: String,
    jmp: Option<String>,
}

impl CTriple {
    pub fn from_string(str: &str) -> PResult<CTriple> {
        tuple((
            opt(terminated(spaced(alphanumeric1), tag("="))),
            spaced(cexpr),
            opt(preceded(tag(";"), spaced(jexpr))),
        ))
        .map(|(src, dst, jmp)| Self {
            dst: src.map(str::to_string),
            expr: dst.to_string(),
            jmp: jmp.map(str::to_string),
        })
        .parse(str)
    }

    pub fn to_cinstr(&self) -> Instruction {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::parse::cinstr::CTriple;

    #[test]
    fn test_make_c_triple() {
        // check that a c instruction with no jmp works
        assert_eq!(
            CTriple::from_string("DM=M+D"),
            Ok((
                "",
                CTriple {
                    dst: Some("DM".to_string()),
                    expr: "M+D".to_string(),
                    jmp: None,
                }
            ))
        );

        // check that a c instruction with jmp works
        assert_eq!(
            CTriple::from_string("DM=M+D;jmp"),
            Ok((
                "",
                CTriple {
                    dst: Some("DM".to_string()),
                    expr: "M+D".to_string(),
                    jmp: Some("jmp".to_string())
                }
            ))
        );

        assert_eq!(
            CTriple::from_string("DM=M+D;jmp\n dee"),
            Ok((
                "\n dee",
                CTriple {
                    dst: Some("DM".to_string()),
                    expr: "M+D".to_string(),
                    jmp: Some("jmp".to_string())
                }
            ))
        );

        // nothing except for expression
        assert_eq!(
            CTriple::from_string("M"),
            Ok((
                "",
                CTriple {
                    dst: None,
                    expr: "M".to_string(),
                    jmp: None,
                }
            ))
        )
    }
}
