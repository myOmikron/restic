use thiserror::Error;

use crate::commands::common::CommandError;

pub type ResticResult<T> = Result<T, ResticError>;

/// Error for the whole crate
#[derive(Debug, Error)]
pub enum ResticError {
    #[error("restic binary is not available")]
    BinaryUnavailable,
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Command(#[from] CommandError),
}
