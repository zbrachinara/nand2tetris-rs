use nom::Err;
use nom::error::ErrorKind;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum AssemblyError {
    #[error("The assembler had an internal problem -- please report")]
    Incomplete,
    #[error("The assembler had an internal problem -- please report")]
    Internal(String, ErrorKind, Option<Box<Self>>),
}

impl nom::error::ParseError<&str> for AssemblyError {
    fn from_error_kind(s: &str, e: ErrorKind) -> Self {
        Self::Internal(s.to_string(), e, None)
    }

    fn append(s: &str, e: ErrorKind, other: Self) -> Self {
        Self::Internal(s.to_string(), e, Some(Box::new(other)))
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
