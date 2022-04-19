use crate::err::AssemblyError;
use crate::parse::PResult;
use nom::Parser;
use std::marker::PhantomData;

struct ManyIterator<'a, Parser, Output> {
    parser: Parser,
    data: &'a str,
    output: PhantomData<Output>,
}

impl<'a, P: Parser<&'a str, O, AssemblyError>, O: 'a> Iterator for ManyIterator<'a, P, O>
// where
//     P: Parser<&'a str, O, AssemblyError>,
{
    type Item = PResult<'a, O>;
    // type Item = Result<O, AssemblyError>;

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
    data: &'a str,
) -> impl Iterator<Item = PResult<O>> + 'a
where
    P: Parser<&'a str, O, AssemblyError>,
{
    ManyIterator {
        parser,
        data,
        output: PhantomData::default(),
    }
}
