use serde::Deserialize;
use serde::Serialize;
use tokio::process::Command;

use crate::Restic;
use crate::commands::common::parse_common;
use crate::error::ResticResult;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InitParams {
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InitOutput {
    pub message_type: String,
    pub id: String,
    pub repository: String,
}

impl Restic {
    pub async fn init(&self, params: InitParams) -> ResticResult<InitOutput> {
        let output = Command::new(&self.restic_path)
            .args(["init", "--json"])
            .env_clear()
            .env("RESTIC_PASSWORD", params.password)
            .spawn()?
            .wait_with_output()
            .await?;

        Ok(parse_common::<InitOutput, ()>(output).await?.unwrap())
    }
}
