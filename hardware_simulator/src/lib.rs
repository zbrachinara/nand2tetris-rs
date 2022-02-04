#![allow(dead_code)] // TODO: remove

mod ir;
mod model;
mod bus_range;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
