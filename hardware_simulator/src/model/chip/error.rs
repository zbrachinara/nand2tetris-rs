use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelConstructionError {
    #[error("Chip cannot be found with the given path")]
    ChipNotFound,
    #[error("An error occurred while parsing the file containing the chip")]
    HdlParseError, //TODO: Include ErrorTree with the error
    #[error("An error occurred while building the model from a file")]
    ConstructionError,
}