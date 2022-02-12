pub mod bus_range;
mod clock_behavior;
pub mod model;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
