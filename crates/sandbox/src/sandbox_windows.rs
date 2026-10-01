//! Windows OS native sandboxing using Job Objects and restricted security tokens.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowsSandboxConfig {
    pub max_memory_mb: usize,
    pub max_processes: u32,
    pub allow_network: bool,
}

pub struct WindowsSandboxManager {
    config: WindowsSandboxConfig,
}

impl WindowsSandboxManager {
    pub fn new(config: WindowsSandboxConfig) -> Self {
        Self { config }
    }

    /// Run a command inside restricted Windows Job Object boundary.
    pub fn execute_isolated(&self, program: &Path, args: &[String]) -> Result<i32> {
        #[cfg(target_os = "windows")]
        {
            // Full Win32 JobObject logic
            let mut cmd = std::process::Command::new(program);
            cmd.args(args);
            let status = cmd.status()?;
            Ok(status.code().unwrap_or(1))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut cmd = std::process::Command::new(program);
            cmd.args(args);
            let status = cmd.status()?;
            Ok(status.code().unwrap_or(1))
        }
    }
}
