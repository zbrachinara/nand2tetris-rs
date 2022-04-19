use crate::err::AssemblyError;
use nom::IResult;
use nom_locate::LocatedSpan;
use crate::assemble::{Address, SymbolTable};
use crate::parse::{Item, Program};

mod space;
mod cinstr;
mod parsing;
mod util;
pub mod structs;

pub type Span<'a> = LocatedSpan<&'a str>;
type PResult<'a, I> = IResult<Span<'a>, I, AssemblyError>;

pub fn program(program: &str) -> Result<(Program, SymbolTable), AssemblyError> {
    let mut sym_table = SymbolTable::new();

    let mut line = 0;

    let program = parsing::program(program.into())
        .map(|res| res.map_err(nom::Err::into).map(|(_, p)| p))
        .filter_map(|item| match item {
            Ok(Item::Label(lb)) => {
                sym_table.insert(lb, Address::Rom(line));
                None
            }
            Ok(Item::Instruction(x)) => {
                line += 1;
                Some(Ok(x))
            }
            Err(x) => Some(Err(x)),
        })
        .try_collect::<Vec<_>>()
        .map(Program);

    program.map(|p| (p, sym_table))
}
