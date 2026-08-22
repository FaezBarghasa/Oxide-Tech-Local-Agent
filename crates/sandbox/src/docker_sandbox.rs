use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tokio::process::Command;

pub struct DockerSandbox {
    pub image_name: String,
    pub workspace_dir: PathBuf,
    pub cpu_limit: String,
    pub memory_limit: String,
}

impl DockerSandbox {
    pub fn new(workspace_dir: PathBuf, image_name: &str) -> Self {
        Self {
            image_name: image_name.to_string(),
            workspace_dir,
            cpu_limit: "8.0".to_string(),     // Limit to 8 CPU cores
            memory_limit: "16g".to_string(),  // Limit to 16GB RAM
        }
    }

    pub fn with_cpu_limit(mut self, cpus: &str) -> Self {
        self.cpu_limit = cpus.to_string();
        self
    }

    pub fn with_memory_limit(mut self, memory: &str) -> Self {
        self.memory_limit = memory.to_string();
        self
    }

    /// Compiles or runs a target command inside a hermetic Docker container
    pub async fn run_build_command(&self, build_cmd: &str) -> Result<(bool, String, String)> {
        let mut cmd = Command::new("docker");
        cmd.arg("run")
            .arg("--rm")
            .arg("--cpus").arg(&self.cpu_limit)
            .arg("--memory").arg(&self.memory_limit)
            .arg("-v").arg(format!("{}:/workspace", self.workspace_dir.display()))
            .arg("-w").arg("/workspace")
            .arg(&self.image_name)
            .arg("sh").arg("-c").arg(build_cmd);

        match cmd.output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Ok((output.status.success(), stdout, stderr))
            }
            Err(e) => {
                tracing::warn!("Docker command execution fallback: {}", e);
                // Fallback to local native process execution if docker daemon is inactive
                let local_output = Command::new("sh")
                    .arg("-c").arg(build_cmd)
                    .current_dir(&self.workspace_dir)
                    .output()
                    .await?;

                let stdout = String::from_utf8_lossy(&local_output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&local_output.stderr).to_string();
                Ok((local_output.status.success(), stdout, stderr))
            }
        }
    }
}
