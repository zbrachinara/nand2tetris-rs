use crate::parse::{CExpr, Dst, Instruction, JumpCondition, Source};
use crate::parse_spanned::space::spaced;
use crate::parse_spanned::{PResult, Span};
use nom::bytes::complete::is_a;
use nom::character::complete::{alpha1, char};
use nom::combinator::opt;
use nom::sequence::{preceded, terminated, tuple};
use nom::Parser;
use std::str::FromStr;
#[allow(clippy::enum_glob_use)]
use CExpr::*;
use Source::{Memory, Register};

fn remove_whitespace(s: Span) -> String {
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
    pub fn from_string(str: Span) -> PResult<CTriple> {
        tuple((
            opt(terminated(spaced(is_a("AMD ")), char('='))),
            spaced(is_a("AMD+-01|&! ")),
            opt(preceded(char(';'), spaced(alpha1))),
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
        let dst = self.dst.as_ref().map_or(Dst::empty(), |str| {
            let mut flags = Dst::empty();
            if str.contains('A') {
                flags |= Dst::A;
            }
            if str.contains('M') {
                flags |= Dst::M;
            }
            if str.contains('D') {
                flags |= Dst::D;
            }
            flags
        });

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
        let jump = self.jmp.as_ref().map_or(Ok(JumpCondition::Never), |s| {
            JumpCondition::from_str(s.as_str()).map_err(|_| "Malformed jump expression")
        })?;

        Ok(Instruction::C { dst, expr, jump })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_c_triple() {
        // check that a c instruction with no jmp works
        assert!(matches!(
            CTriple::from_string(Span::from("DM=M+D")),
            Ok((
                rem,
                CTriple {
                    dst: Some(d),
                    expr: e,
                    jmp: None,
                }
            )) if *rem == "" && d == "DM" && e == "M+D"
        ));

        // check that a c instruction with jmp works
        assert!(matches!(
            CTriple::from_string(Span::from("   D   M     =M+D;JMP")),
            Ok((
                rem,
                CTriple {
                    dst: Some(d),
                    expr: e,
                    jmp: Some(j),
                }
            )) if *rem == "" && d == "DM" && e == "M+D" && j == "JMP"
        ));

        assert!(matches!(
            CTriple::from_string(Span::from("DM=    M    +   D  \t;JMP\n dee")),
            Ok((
                rem,
                CTriple {
                    dst: Some(d),
                    expr: e,
                    jmp: Some(j),
                }
            )) if *rem == "\n dee" && d == "DM" && e == "M+D" && j == "JMP"
        ));

        // nothing except for expression
        assert!(matches!(
            CTriple::from_string(Span::from("M")),
            Ok((
                res,
                CTriple {
                    dst: None,
                    expr: e,
                    jmp: None,
                }
            )) if *res == "" && e == "M"
        ));

        // some other test cases
        assert!(matches!(
            CTriple::from_string(Span::from("M=0")),
            Ok((
                res,
                CTriple {
                    dst: Some(d),
                    expr: e,
                    jmp: None,
                }
            )) if *res == "" && d == "M" && e == "0"
        ));

        assert!(matches!(
            CTriple::from_string(Span::from("D;JMP")),
            Ok((
                res,
                CTriple {
                    dst: None,
                    expr: e,
                    jmp: Some(j),
                }
            )) if *res == "" && e == "D" && j == "JMP"
        ));
    }
}
