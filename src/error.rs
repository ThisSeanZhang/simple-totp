use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum TOTPError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    
    // #[error("data path error")]
    // DataPathError,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, TOTPError>;
