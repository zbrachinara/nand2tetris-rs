use crate::parse::space::spaced;
use crate::parse::{CExpr, Dst, Instruction, JumpCondition, PResult, Source};
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::sequence::{preceded, terminated, tuple};
use nom::Parser;
use nom_supreme::tag::complete::tag;
use std::str::FromStr;

fn aexpr(str: &str) -> PResult<&str> {
    fn is_aexpr_char(c: char) -> bool {
        matches!(c, 'A' | 'M' | 'D' | ' ')
    }
    take_while1(is_aexpr_char).parse(str)
}

fn cexpr(str: &str) -> PResult<&str> {
    fn is_cexpr_char(c: char) -> bool {
        matches!(c, 'A' | 'M' | 'D' | '+' | '-' | ' ' | '0' | '1' | '|' | '&')
    }
    take_while1(is_cexpr_char).parse(str)
}

fn jexpr(str: &str) -> PResult<&str> {
    take_while1(|c: char| c.is_ascii_uppercase())(str)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
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
            opt(terminated(spaced(aexpr), tag("="))),
            spaced(cexpr),
            opt(preceded(tag(";"), spaced(jexpr))),
        ))
        .map(|(dst, expr, jmp)| {
            (
                dst.map(remove_whitespace),
                remove_whitespace(expr),
                jmp.map(remove_whitespace),
            )
        })
        .map(|(dst, expr, jmp)| Self { dst, expr, jmp })
        .parse(str)
    }

    pub fn to_cinstr(&self) -> Result<Instruction, String> {
        // convert destination
        let dst = self
            .dst
            .as_ref()
            .map(|str| {
                let mut flags = Dst::empty();
                if str.contains("A") {
                    flags |= Dst::A
                }
                if str.contains("M") {
                    flags |= Dst::M
                }
                if str.contains("D") {
                    flags |= Dst::D
                }
                flags
            })
            .unwrap_or(Dst::empty());

        use CExpr::*;
        use Source::*;
        // convert expression
        let expr = match self.expr.as_str() {
            // constants
            "0" => Ok(Zero),
            "1" => Ok(One),
            "-1" => Ok(NegOne),

            // evaluate to value
            "D" => Ok(D),
            "A" => Ok(X(Register)),
            "M" => Ok(X(Memory)),

            // unary operation
            "!D" => Ok(NotD),
            "!A" => Ok(NotX(Register)),
            "!M" => Ok(NotX(Memory)),

            "-D" => Ok(NegD),
            "-A" => Ok(NegX(Register)),
            "-M" => Ok(NegX(Memory)),

            // inc/decrement (for some reason can't specify them backward)
            "D+1" => Ok(DPlusOne),
            "A+1" => Ok(XPlusOne(Register)),
            "M+1" => Ok(XPlusOne(Memory)),

            "D-1" => Ok(DMinusOne),
            "A-1" => Ok(XMinusOne(Register)),
            "M-1" => Ok(XMinusOne(Memory)),

            // binary operators
            "D+A" | "A+D" => Ok(DPlusX(Register)),
            "D+M" | "M+D" => Ok(DPlusX(Memory)),

            "D-A" => Ok(DMinusX(Register)),
            "D-M" => Ok(DMinusX(Memory)),

            "A-D" => Ok(XMinusD(Register)),
            "M-D" => Ok(XMinusD(Memory)),

            "D&A" | "A&D" => Ok(DAndX(Register)),
            "D&M" | "M&D" => Ok(DAndX(Memory)),

            "D|A" | "A|D" => Ok(DOrX(Register)),
            "D|M" | "M|D" => Ok(DOrX(Memory)),

            _ => Err("Malformed Compute expression"),
        }?;

        // convert jump directive
        let jump = self
            .jmp
            .as_ref()
            .map(|s| JumpCondition::from_str(s.as_str()).map_err(|_| "Malformed jump expression"))
            .unwrap_or(Ok(JumpCondition::Never))?;

        Ok(Instruction::C { dst, expr, jump })
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
            CTriple::from_string("   D   M     =M+D;JMP"),
            Ok((
                "",
                CTriple {
                    dst: Some("DM".to_string()),
                    expr: "M+D".to_string(),
                    jmp: Some("JMP".to_string())
                }
            ))
        );

        assert_eq!(
            CTriple::from_string("DM=    M    +   D  \t;JMP\n dee"),
            Ok((
                "\n dee",
                CTriple {
                    dst: Some("DM".to_string()),
                    expr: "M+D".to_string(),
                    jmp: Some("JMP".to_string())
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
        );

        // some other test cases
        assert_eq!(
            CTriple::from_string("M=0"),
            Ok((
                "",
                CTriple {
                    dst: Some("M".to_string()),
                    expr: "0".to_string(),
                    jmp: None,
                }
            ))
        );

        assert_eq!(
            CTriple::from_string("D;JMP"),
            Ok((
                "",
                CTriple {
                    dst: None,
                    expr: "D".to_string(),
                    jmp: Some("JMP".to_string())
                }
            ))
        )
    }
}
