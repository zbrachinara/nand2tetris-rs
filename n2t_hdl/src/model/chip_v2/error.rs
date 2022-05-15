use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelConstructionError {
    #[error("Chip depends on {0}, but cannot find it")]
    Needs(String),
    #[error("Chip {0} was already loaded into the builder, but was just loaded again")]
    Rebuilt(String),
}
