use nom::Err;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum AssemblyError {
    #[error("The assembler had an internal problem -- please report")]
    Internal,
}

impl From<nom::Err<AssemblyError>> for AssemblyError {
    fn from(nom_error: Err<AssemblyError>) -> Self {
        match nom_error {
            Err::Incomplete(_) => AssemblyError::Internal,
            Err::Error(e) | Err::Failure(e) => e,
        }
    }
}
