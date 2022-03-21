use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelConstructionError {
    #[error("Chip `{0}` cannot be found with the given path")]
    ChipNotFound(String),

    #[error("An error occurred while parsing the file containing the chip")]
    HdlParseError, //TODO: Include ErrorTree with the error

    #[error("An error occurred while building the model from a file")]
    ConstructionError,

    #[error("An unknown error occurred")]
    Unk(Option<anyhow::Error>),

    #[error("An error occurred (but we can't tell what)")]
    Alien,
}
