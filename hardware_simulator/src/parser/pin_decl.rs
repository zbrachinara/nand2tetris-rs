use nom::character::complete::{char, digit1, multispace0};
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn bus_declaration(arg: &str) -> IResult<&str, u16> {
    let (remainder, size) = delimited(
        tuple((multispace0, char('['), multispace0)),
        digit1,
        tuple((multispace0, char(']'), multispace0)),
    )(arg)?;

    Ok((remainder, u16::from_str_radix(size, 10).unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_declaration() {
        assert_eq!(bus_declaration("[1]"), Ok(("", 1)));
        assert_eq!(bus_declaration("[5]"), Ok(("", 5)));
        assert_eq!(bus_declaration("[25]"), Ok(("", 25)));
        assert_eq!(bus_declaration("\n[\n25\n]\n"), Ok(("", 25)));
        assert_eq!(bus_declaration("\n[\n25\n]\nbruh"), Ok(("bruh", 25)));
    }
}
