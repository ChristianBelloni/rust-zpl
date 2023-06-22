use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unsupported command")]
    UnsupportedCommand,
    #[error("failed to parse integer")]
    ParseInt(#[from] ParseIntError),
    #[error("missing parameter name: {0}")]
    MissingParameter(String),
}
