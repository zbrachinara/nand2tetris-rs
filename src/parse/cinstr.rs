use crate::parse::space::spaced;
use crate::parse::{CExpr, Dst, Instruction, JumpCondition, PResult, Source};
use nom::branch::alt;
use nom::bytes::complete::{take_until, take_while1};
use nom::character::complete::{alphanumeric1, char};
use nom::character::is_alphabetic;
use nom::combinator::opt;
use nom::sequence::{preceded, terminated, tuple};
use nom::{AsChar, Parser};
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
                str.contains("A").then(|| Dst::A).unwrap_or(Dst::empty())
                    | str.contains("M").then(|| Dst::M).unwrap_or(Dst::empty())
                    | str.contains("D").then(|| Dst::D).unwrap_or(Dst::empty())
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
            CTriple::from_string("   D   M     =M+D;jmp"),
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
            CTriple::from_string("DM=    M    +   D  \t;jmp\n dee"),
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
