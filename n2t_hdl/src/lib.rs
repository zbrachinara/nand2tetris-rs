pub mod channel_range;
mod clock_behavior;
pub mod model;
pub mod prelude;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
