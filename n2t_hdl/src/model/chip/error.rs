use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelConstructionError {
    #[error("Chip depends on {0:?}, but cannot find it")]
    Needs(Vec<String>),
    #[error("Chip {0} was already loaded into the builder, but was just loaded again")]
    Rebuilt(String),
    #[error("Pin {0} was set to a number, which is not supported as of yet")]
    ValuesNotSupported(String),
    #[error("Pin {failed_internal} was expected to have size {expected}, but {failed_external} has size {actual}")]
    MismatchedSizes {
        failed_internal: String,
        failed_external: String,
        expected: usize,
        actual: usize,
    },
    #[error("The pin {0} could not be found in the chip {0}")]
    PinNotFound(String, String),
    #[error("The pin {0} has two sources")]
    ConflictingSources(String),
    #[error("The pin {0} has no source")]
    NoSource(String),
}
