#![feature(iterator_try_collect)]
#![feature(let_else)]

pub mod channel_range;
mod clock_behavior;
pub mod model;
pub mod prelude;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
