use thiserror::Error;


#[derive(Debug, Error)]
pub enum ModelConstructionError {
    #[error("Chip depends on {0}, but cannot find it")]
    Needs(String),
}