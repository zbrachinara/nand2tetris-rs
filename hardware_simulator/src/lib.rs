mod parser;
mod model;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
