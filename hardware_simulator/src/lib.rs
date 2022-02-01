#![allow(dead_code)] // TODO: remove

mod parser;
mod model;
mod ir;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, Eq, PartialEq)]
pub struct BusRange {
    pub start: u16,
    pub end: u16,
}