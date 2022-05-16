use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelConstructionError {
    #[error("Chip depends on {0}, but cannot find it")]
    Needs(String),
    #[error("Chip {0} was already loaded into the builder, but was just loaded again")]
    Rebuilt(String),
    #[error("Pin {0} was set to a value, which is not supported as of yet")]
    ValuesNotSupported(String),
    #[error("")]
    MismatchedSizes {
        failed: String,
        expected: usize,
        actual: usize,
    },
}
