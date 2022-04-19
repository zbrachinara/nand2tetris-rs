use crate::err::AssemblyError;
use nom::IResult;
use nom_locate::LocatedSpan;

mod space;

type Span<'a> = LocatedSpan<&'a str>;
type PResult<'a, I> = IResult<Span<'a>, I, AssemblyError>;
