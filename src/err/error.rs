use crate::err::str;
use nom::error::ErrorKind;
use nom::Err;
use crate::parse_spanned::Span;
use nom::error::ParseError;

#[derive(Debug, PartialEq, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum AssemblyError {
    #[error("A problem was encountered while parsing an identifier")]
    InvalidIdentifier,
    #[error("A problem was detected while parsing a compute instruction")]
    InvalidCExpr,
    #[error("The assembler had an internal problem (Incomplete) -- please report")]
    Incomplete,
    #[error("The assembler had an internal problem -- please report")]
    Internal(String, ErrorKind, Option<Box<Self>>),
}

impl ParseError<&str> for AssemblyError {
    fn from_error_kind(s: &str, e: ErrorKind) -> Self {
        Self::Internal(s.to_string(), e, None)
    }

    fn append(s: &str, e: ErrorKind, other: Self) -> Self {
        Self::Internal(s.to_string(), e, Some(Box::new(other)))
    }
}

impl <'a> ParseError<Span<'a>> for AssemblyError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        <Self as ParseError<&str>>::from_error_kind(*input, kind)
    }

    fn append(input: Span, kind: ErrorKind, other: Self) -> Self {
        <Self as ParseError<&str>>::append(*input, kind, other)
    }
}

impl AssemblyError {
    pub fn raise(self) -> Self {
        match self {
            Self::Internal(_, _, Some(inner)) => inner.raise(),
            x => x,
        }
    }

    pub fn trace(&self) {
        match self {
            Self::Internal(str, err, x) => {
                eprintln!("Internal error: {}, {err:?}", str::legible_string(str));
                if let Some(this) = x {
                    eprint!("Caused by: ");
                    this.trace();
                }
            }
            x => eprintln!("{x}"),
        }
    }
}

impl From<nom::Err<AssemblyError>> for AssemblyError {
    fn from(nom_error: Err<AssemblyError>) -> Self {
        match nom_error {
            Err::Incomplete(_) => AssemblyError::Incomplete,
            Err::Error(e) | Err::Failure(e) => e,
        }
    }
}
