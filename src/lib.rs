//! # restic
//!
//! Wrapper for the cli tool `restic`

use std::fs;

use tracing::debug;
use tracing::info;

use crate::error::ResticError;
use crate::error::ResticResult;

pub mod commands;
pub mod error;

#[derive(Debug)]
pub struct Restic {
    restic_path: String,
}

impl Restic {
    /// Create a new restic instance
    ///
    /// `restic_path`: Path to the binary where `restic` can be found
    pub async fn new(restic_path: String) -> ResticResult<Restic> {
        fs::exists(&restic_path).map_err(|_| ResticError::BinaryUnavailable)?;

        let restic = Restic { restic_path };
        info!("Testing restic capabilities by calling restic version");
        let version = restic.version().await?;
        debug!("Restic version: {:?}", version);

        Ok(restic)
    }
}
