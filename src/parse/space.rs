use nom::branch::alt;
use nom::bytes::complete::{is_not, take_until};
use nom::character::complete::multispace1;
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use nom_supreme::tag::complete::tag;

fn generic_space1(arg: &str) -> IResult<&str, ()> {
    many0(alt((
        multispace1,
        complete(delimited(tag("/*"), take_until("*/"), tag("*/"))),
        complete(preceded(tag("//"), is_not("\n"))),
    )))
    .map(|_| ())
    .parse(arg)
}

pub fn generic_space0(arg: &str) -> IResult<&str, ()> {
    opt(generic_space1).map(|_| ()).parse(arg)
}

pub fn spaced<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(generic_space0, inner, generic_space0)
}