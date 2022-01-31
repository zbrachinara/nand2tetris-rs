use crate::parser::{name, spaced, Builtin, Connection, Implementation, PResult, Span};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom_supreme::tag::complete::tag;

fn builtin(arg: Span) -> PResult<Builtin> {
    let (remainder, (name, clocked)) = tuple((
        spaced(delimited(tag("BUILTIN"), name, char(';'))),
        opt(spaced(delimited(
            tag("CLOCKED"),
            separated_list0(char(','), name),
            char(';'),
        ))),
    ))(arg)?;

    Ok((remainder, Builtin { name, clocked }))
}

fn native(_: Span) -> PResult<Vec<Connection>> {
    todo!()
}

fn implementation(_: Span) -> PResult<Implementation> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builtin() {
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     CLOCKED in;")).unwrap();
            let (remainder, Builtin {name, clocked}) = res;
            assert_eq!(*remainder, "");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, Some(_)));
            if let Some(clocked) = clocked {
                assert_eq!(*(clocked[0]), "in");
            }
        }
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     CLOCKED in, out;")).unwrap();
            let (remainder, Builtin {name, clocked}) = res;
            assert_eq!(*remainder, "");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, Some(_)));
            if let Some(clocked) = clocked {
                assert_eq!(*(clocked[0]), "in");
                assert_eq!(*(clocked[1]), "out");
            }
        }
        {
            let res = builtin(Span::new("BUILTIN DFF;\n     // CLOCKED in;")).unwrap();
            let (remainder, Builtin {name, clocked}) = res;
            assert_eq!(*remainder, "");
            assert_eq!(*name, "DFF");

            assert!(matches!(clocked, None));
        }
    }
}
