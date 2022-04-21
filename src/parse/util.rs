use crate::err::AssemblyError;
use crate::parse::{PResult, Span};
use nom::{InputTake, InputTakeAtPosition, Parser};
use std::marker::PhantomData;

struct ManyIterator<'a, Parser, Output> {
    parser: Parser,
    data: Span<'a>,
    split_by: char,
    output: PhantomData<Output>,
    done: bool,
}

impl<'a, P: Parser<Span<'a>, O, AssemblyError>, O: 'a> Iterator for ManyIterator<'a, P, O> {
    type Item = PResult<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            Some(match self.data.split_at_position::<_, AssemblyError>(|c| c == self.split_by) {
                Ok((next, this)) => {
                    self.data = next.take_split(1).0;
                    self.parser.parse(this)
                }
                Err(nom::Err::Incomplete(_)) => {
                    self.done = true;
                    self.parser.parse(self.data)
                }
                _ => unreachable!(),
            })
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
        parser,
        data,
        split_by,
        output: PhantomData::default(),
        done: false,
    }
}
