use std::process::Output;

use serde::de::DeserializeOwned;
use thiserror::Error;

/// Parse the common errors from the give output
pub async fn parse_common<Ok, Err>(output: Output) -> Result<Result<Ok, Err>, CommandError>
where
    Ok: DeserializeOwned,
    Err: DeserializeOwned,
{
    match output.status.code() {
        // Program was interrupted by a signal
        None => Err(CommandError::Interrupted),
        Some(0) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let data: Ok = serde_json::from_str(&stdout)?;
            Ok(Ok(data))
        }
        Some(1) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let data: Err = serde_json::from_str(&stderr)?;
            Ok(Err(data))
        }
        Some(2) => Err(CommandError::GoRuntimeError),
        Some(3) => Err(CommandError::CouldNotReadSourceData),
        Some(10) => Err(CommandError::RepositoryDoesNotExist),
        Some(11) => Err(CommandError::FailedToLockRepository),
        Some(12) => Err(CommandError::WrongPassword),
        Some(130) => Err(CommandError::Interrupted),
        _ => Err(CommandError::Unknown(
            String::from_utf8_lossy(&output.stderr).to_string(),
        )),
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Go runtime error")]
    GoRuntimeError,
    #[error("Could not read source data")]
    CouldNotReadSourceData,
    #[error("Repository does not exist")]
    RepositoryDoesNotExist,
    #[error("Failed to lock repository")]
    FailedToLockRepository,
    #[error("Wrong password")]
    WrongPassword,
    #[error("Restic was interrupted using SIGINT or SIGSTOP")]
    Interrupted,
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Deserialize error: {0}")]
    Serde(#[from] serde_json::Error),
}
