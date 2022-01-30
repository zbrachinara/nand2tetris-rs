use nom::IResult;
use crate::parser::{Builtin, Connection, Implementation};

fn builtin(_: &str) -> IResult<&str, Builtin> {
    todo!()
}

fn native(_: &str) -> IResult<&str, Vec<Connection>> {
    todo!()
}

fn implementation(_: &str) -> IResult<&str, Implementation> {
    todo!()
}

#[cfg(test)]
mod test {

}