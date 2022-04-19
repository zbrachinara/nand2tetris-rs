use crate::parse_spanned::{PResult, Span};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::Parser;
use crate::err::AssemblyError;

fn generic_space1(arg: Span) -> PResult<Vec<Span>> {
    many0(alt((space1, complete(preceded(tag("//"), is_not("\n")))))).parse(arg)
}

pub fn generic_space0(arg: Span) -> PResult<Option<Vec<Span>>> {
    opt(generic_space1).parse(arg)
}

fn line_space1(arg: Span) -> PResult<Vec<Span>> {
    many0(alt((
        space1,
        line_ending,
        complete(preceded(tag("//"), is_not("\n"))),
    )))
    .parse(arg)
}

fn line_space0(arg: Span) -> PResult<Option<Vec<Span>>> {
    opt(line_space1).parse(arg)
}

pub fn spaced<'a, P0, O>(inner: P0) -> impl Parser<Span<'a>, O, AssemblyError>
where
    P0: Parser<Span<'a>, O, AssemblyError>,
    // P1: Parser<Span<'a>, O, E>,
{
    delimited(generic_space0, inner, generic_space0)
}

pub fn line_spaced<'a, P0, O>(inner: P0) -> impl Parser<Span<'a>, O, AssemblyError>
where
    P0: Parser<Span<'a>, O, AssemblyError>,
{
    delimited(line_space0, inner, line_space0)
}
