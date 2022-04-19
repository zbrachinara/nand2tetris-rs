use crate::err::AssemblyError;
use crate::parse_spanned::{PResult, Span};
use nom::Parser;
use std::marker::PhantomData;

struct ManyIterator<'a, Parser, Output> {
    parser: Parser,
    data: Span<'a>,
    output: PhantomData<Output>,
}

impl<'a, P: Parser<Span<'a>, O, AssemblyError>, O: 'a> Iterator for ManyIterator<'a, P, O> {
    type Item = PResult<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            match self.parser.parse(self.data) {
                PResult::Ok((rem, out)) => {
                    self.data = rem;
                    Some(Ok((rem, out)))
                }
                PResult::Err(err) => Some(Err(err)),
            }
        }
    }
}

pub fn many0_iterate<'a, P: 'a, O: 'a>(
    parser: P,
    data: Span<'a>,
) -> impl Iterator<Item = PResult<O>> + 'a
where
    P: Parser<Span<'a>, O, AssemblyError>,
{
    ManyIterator {
        parser,
        data,
        output: PhantomData::default(),
    }
}
