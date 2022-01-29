use nom::IResult;

fn bus_declaration(_: &str) -> IResult<&str, u16> {
    todo!()
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