use crate::err::AssemblyError;
use crate::parse_spanned::{PResult, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_a, is_not};
use nom::character::complete::{line_ending, space1};
use nom::combinator::{complete, opt};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded};
use nom::Parser;

fn generic_space1(arg: Span) -> PResult<()> {
    fold_many0(
        alt((space1, complete(preceded(tag("//"), is_not("\n"))))),
        || (),
        |_, _| (),
    )
    .parse(arg)
}

pub fn generic_space0(arg: Span) -> PResult<Option<()>> {
    opt(generic_space1).parse(arg)
}

fn line_space1(arg: Span) -> PResult<()> {
    fold_many0(
        alt((
            space1,
            line_ending,
            is_a("\r"),
            complete(preceded(tag("//"), is_not("\r\n"))),
        )),
        || (),
        |_, _| (),
    )
    .parse(arg)
}

fn line_space0(arg: Span) -> PResult<Option<()>> {
    opt(line_space1).parse(arg)
}

pub fn spaced<'a, P0, O>(inner: P0) -> impl Parser<Span<'a>, O, AssemblyError>
where
    P0: Parser<Span<'a>, O, AssemblyError>,
{
    delimited(generic_space0, inner, generic_space0)
}

#[allow(dead_code)]
pub fn line_spaced<'a, P0, O>(inner: P0) -> impl Parser<Span<'a>, O, AssemblyError>
where
    P0: Parser<Span<'a>, O, AssemblyError>,
{
    delimited(line_space0, inner, line_space0)
}

pub fn alt_line_spaced<'a, P0, O>(mut inner: P0) -> impl Parser<Span<'a>, Option<O>, AssemblyError>
where
    P0: Parser<Span<'a>, O, AssemblyError>,
{
    move |s: Span<'a>| {
        let (remaining, _) = line_space0(s)?;
        if remaining.is_empty() {
            Ok((remaining, None))
        } else {
            let (remaining, out) = inner.parse(remaining)?;
            let (remaining, _) = line_space0.parse(remaining)?;
            Ok((remaining, Some(out)))
        }
    }
}
