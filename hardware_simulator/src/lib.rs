mod ir;
pub mod model;
pub mod bus_range;
mod clock_behavior;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
