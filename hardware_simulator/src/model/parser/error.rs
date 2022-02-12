use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HdlParseError {
    #[error("Not a valid symbol")]
    BadSymbol,
    #[error("Name is not correct (Must not be a number or literal)")]
    BadName,
    #[error("Number is too large")]
    NumberOverflow,
    #[error("A problem occurred when trying to parse this number")]
    NumberError,
    #[error("Could not deduce a given implementation")]
    BadImplementation,
}
