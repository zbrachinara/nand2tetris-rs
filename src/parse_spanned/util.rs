use crate::err::AssemblyError;
use crate::parse_spanned::{PResult, Span};
use nom::combinator::opt;
use nom::{InputTakeAtPosition, Parser};
use std::marker::PhantomData;

struct ManyIterator<'a, Parser, Output> {
    parser: Parser,
    data: Span<'a>,
    split_by: char,
    output: PhantomData<Output>,
}

impl<'a, P: Parser<Span<'a>, Option<O>, AssemblyError>, O: 'a> Iterator for ManyIterator<'a, P, O> {
    type Item = PResult<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            self.data
                .split_at_position(|c| c == self.split_by)
                .and_then(|(next, this)| {
                    self.data = next;
                    self.parser.parse(this).map(|(x, y)| y.map(|y| (x, y)))
                })
                .transpose()
        }
    }
}

pub fn many0_spliterate<'a, P: 'a, O: 'a>(
    parser: P,
    data: Span<'a>,
    split_by: char,
) -> impl Iterator<Item = PResult<O>> + 'a
where
    P: Parser<Span<'a>, O, AssemblyError>,
{
    ManyIterator {
        parser: opt(parser),
        data,
        split_by,
        output: PhantomData::default(),
    }
}
