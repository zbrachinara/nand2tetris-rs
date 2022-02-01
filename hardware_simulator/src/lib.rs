mod parser;
mod model;
mod ir;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
