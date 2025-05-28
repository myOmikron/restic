use std::process::Command;

use serde::Deserialize;
use serde::Serialize;

use crate::Restic;
use crate::commands::common::parse_common;
use crate::error::ResticResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionOutput {
    pub message_type: String,
    pub version: String,
    pub go_version: String,
    pub go_os: String,
    pub go_arch: String,
}

impl Restic {
    /// Return the version of the restic binary
    pub async fn version(&self) -> ResticResult<VersionOutput> {
        let output = Command::new(&self.restic_path)
            .env_clear()
            .args(["version", "--json"])
            .spawn()?
            .wait_with_output()?;

        let version = parse_common::<VersionOutput, ()>(output).await?.unwrap();

        Ok(version)
    }
}
