use flexstr::{local_str, ToLocalStr};
use nom::error::ErrorKind;
use nom::Err;
use nom_supreme::tag::TagError;

#[derive(Debug, PartialEq, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum AssemblyError {
    #[error("A problem was encountered while parsing an identifier")]
    InvalidIdentifier,
    #[error("A problem was detected while parsing a compute instruction")]
    InvalidCExpr,
    #[error("The assembler had an internal problem -- please report")]
    Incomplete,
    #[error("The assembler had an internal problem -- please report")]
    Internal(String, ErrorKind, Option<Box<Self>>),
}

impl TagError<&str, &str> for AssemblyError {
    fn from_tag(input: &str, _: &str) -> Self {
        Self::Internal(input.to_string(), ErrorKind::Tag, None)
    }
}

impl nom::error::ParseError<&str> for AssemblyError {
    fn from_error_kind(s: &str, e: ErrorKind) -> Self {
        Self::Internal(s.to_string(), e, None)
    }

    fn append(s: &str, e: ErrorKind, other: Self) -> Self {
        Self::Internal(s.to_string(), e, Some(Box::new(other)))
    }
}

fn legible_string(s: &str) -> String {

    let mut len = 0;

    let modified_s = s
        .chars()
        .map(|c| match c {
            '\n' => local_str!(","),
            '\r' => local_str!(""),
            x => x.to_local_str(),
        })
        .take_while(|s| {
            len += s.len();
            len < 200
        })
        .fold(String::with_capacity(205), |mut acc, c| {
            acc.push_str(c.as_str());
            acc
        });

    if s.len() > 200 {
        modified_s + "..."
    } else {
        modified_s
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
                eprintln!("Internal error: {}, {err:?}", legible_string(str));
                if let Some(this) = x {
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
