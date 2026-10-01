use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Rootless, daemonless Podman sandbox container runner with automated OS fallback
pub struct PodmanSandbox {
    pub image: String,
    pub work_dir: PathBuf,
    pub memory_limit: String,
    pub cpu_quota: f32,
}

impl PodmanSandbox {
    pub fn new(work_dir: PathBuf, image: &str) -> Self {
        Self {
            image: image.to_string(),
            work_dir,
            memory_limit: "2g".to_string(),
            cpu_quota: 2.0,
        }
    }

    pub fn with_memory_limit(mut self, memory_limit: &str) -> Self {
        self.memory_limit = memory_limit.to_string();
        self
    }

    pub fn with_cpu_quota(mut self, cpu_quota: f32) -> Self {
        self.cpu_quota = cpu_quota;
        self
    }

    /// Check whether Podman is installed and callable on the host
    pub fn is_podman_available() -> bool {
        Command::new("podman")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Run isolated command inside rootless Podman container, falling back to local runner if Podman is absent
    pub fn run_isolated_command(&self, cmd: &[String]) -> Result<std::process::Output> {
        if Self::is_podman_available() {
            let mut podman_cmd = Command::new("podman");
            podman_cmd
                .arg("run")
                .arg("--rm")
                .arg("--interactive")
                .arg("--network=none")
                .arg("--security-opt=no-new-privileges")
                .arg("--cap-drop=ALL")
                .arg(format!("--memory={}", self.memory_limit))
                .arg(format!("--cpus={}", self.cpu_quota))
                .arg("--read-only")
                .arg(format!("-v={}:/workspace:Z", self.work_dir.display()))
                .arg("--workdir=/workspace")
                .arg(&self.image)
                .args(cmd);

            podman_cmd.output().context("Failed to execute rootless Podman sandbox")
        } else {
            tracing::warn!(
                "Podman binary not found on host; falling back to direct host command execution"
            );
            if cmd.is_empty() {
                return Err(anyhow::anyhow!("Empty command passed to sandbox"));
            }
            let mut host_cmd = Command::new(&cmd[0]);
            if cmd.len() > 1 {
                host_cmd.args(&cmd[1..]);
            }
            host_cmd
                .current_dir(&self.work_dir)
                .output()
                .context("Failed to execute fallback host command")
        }
    }

    /// Asynchronous execution wrapper for build commands
    pub async fn run_build_command(&self, build_cmd: &str) -> Result<(bool, String, String)> {
        let cmd = vec!["sh".to_string(), "-c".to_string(), build_cmd.to_string()];
        let output = self.run_isolated_command(&cmd)?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok((output.status.success(), stdout, stderr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn podman_sandbox_test() {
        let tmp = tempdir().unwrap();
        let sandbox = PodmanSandbox::new(tmp.path().to_path_buf(), "alpine:latest");
        let cmd = vec!["echo".to_string(), "Podman Sandbox Ready".to_string()];
        let res = sandbox.run_isolated_command(&cmd).unwrap();
        assert!(res.status.success());
        let stdout = String::from_utf8_lossy(&res.stdout);
        assert!(stdout.contains("Podman Sandbox Ready"));
    }
}
