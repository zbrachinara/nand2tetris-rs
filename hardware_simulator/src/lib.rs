mod ir;
pub mod model;
pub mod bus_range;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
