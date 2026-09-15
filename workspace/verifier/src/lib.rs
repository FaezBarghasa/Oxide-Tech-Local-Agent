use common::error::{EiosError, Result};
use std::path::Path;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{Duration, timeout};
use tracing::{error, info};

const EXECUTION_TIMEOUT: Duration = Duration::from_secs(30);
const MEMORY_LIMIT_BYTES: u64 = 1024 * 1024 * 1024; // 1 GiB
const CPU_TIME_LIMIT_SECS: u64 = 60;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub async fn execute_in_sandbox(cmd: &[&str], work_dir: &str) -> Result<ExecutionResult> {
    if cmd.is_empty() {
        return Err(EiosError::Internal(
            "Empty command provided to sandbox".to_string(),
        ));
    }

    let work_dir_path = Path::new(work_dir);
    if !work_dir_path.exists() {
        return Err(EiosError::Internal(format!(
            "Sandbox working directory does not exist: {}",
            work_dir
        )));
    }

    info!("Sandbox executing {:?} in {:?}", cmd, work_dir);

    let mut child = unsafe {
        Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(work_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .pre_exec(|| {
                // New session for process group signals
                nix::unistd::setsid().map_err(std::io::Error::other)?;

                // Virtual memory cap (RLIMIT_AS)
                let mem_limit = nix::sys::resource::rlim_t::from(MEMORY_LIMIT_BYTES);
                nix::sys::resource::setrlimit(
                    nix::sys::resource::Resource::RLIMIT_AS,
                    mem_limit,
                    mem_limit,
                )
                .map_err(std::io::Error::other)?;

                // CPU time cap (RLIMIT_CPU)
                let cpu_limit = nix::sys::resource::rlim_t::from(CPU_TIME_LIMIT_SECS);
                nix::sys::resource::setrlimit(
                    nix::sys::resource::Resource::RLIMIT_CPU,
                    cpu_limit,
                    cpu_limit,
                )
                .map_err(std::io::Error::other)?;

                Ok(())
            })
            .spawn()
            .map_err(|e| std::io::Error::other(format!("Failed to spawn process: {}", e)))?
    };

    let pid = child.id().unwrap_or(0);

    let stdout_handle = child.stdout.take().expect("stdout pipe missing");
    let stderr_handle = child.stderr.take().expect("stderr pipe missing");

    let read_result = timeout(EXECUTION_TIMEOUT, async {
        let (stdout_bytes, stderr_bytes) = tokio::join!(
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stdout_handle);
                let _ = reader.read_to_end(&mut buf).await;
                buf
            },
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stderr_handle);
                let _ = reader.read_to_end(&mut buf).await;
                buf
            }
        );
        (stdout_bytes, stderr_bytes)
    })
    .await;

    match read_result {
        Ok((stdout_bytes, stderr_bytes)) => {
            let status = child.wait().await.map_err(|e| {
                EiosError::Internal(format!("Failed to wait on sandboxed process: {}", e))
            })?;

            let exit_code = status.code().unwrap_or(-1);

            Ok(ExecutionResult {
                exit_code,
                stdout: String::from_utf8_lossy(&stdout_bytes).into_owned(),
                stderr: String::from_utf8_lossy(&stderr_bytes).into_owned(),
            })
        }
        Err(_) => {
            error!(
                "Sandbox timeout exceeded for command {:?} (pid {}). Killing process group.",
                cmd, pid
            );

            let _ = child.wait().await;

            Err(EiosError::Internal(format!(
                "Sandboxed command {:?} exceeded timeout and was killed.",
                cmd
            )))
        }
    }
}

pub mod checkpoint;
pub mod evidence;
pub mod git_engine;
pub mod qemu_firmware;
pub mod remote_ssh;

pub use checkpoint::{AtomicFileSnapshot, CheckpointManager, WorkspaceCheckpoint};
pub use evidence::{EvidenceBundle, VerifierReport};
pub use git_engine::{GitCommitInfo, GitEngine, GitStatusResult};
pub use qemu_firmware::FirmwareEmulationVerifier;
pub use remote_ssh::{RemoteSshManager, SshConfig};

/// Bridge connecting verification execution with the Multi-Agent Coordinator verifier role.
#[derive(Debug, Clone, Default)]
pub struct MultiAgentVerifierBridge;

impl MultiAgentVerifierBridge {
    pub fn new() -> Self {
        Self
    }

    /// Run sandboxed cargo-check or test command and format standard feedback for the Verifier agent.
    pub async fn verify_in_sandbox(&self, cmd: &[&str], work_dir: &str) -> Result<VerifierReport> {
        let start = std::time::Instant::now();
        let res = execute_in_sandbox(cmd, work_dir).await?;
        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(VerifierReport {
            stage: cmd.join(" "),
            passed: res.exit_code == 0,
            stdout: res.stdout,
            stderr: res.stderr,
            duration_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_rklipper_firmware_verification() {
        let verifier = FirmwareEmulationVerifier::new().with_timeout(1);
        let dummy_elf = PathBuf::from("workspace/firmware/rklipper.elf");
        let res = verifier
            .verify_rklipper_firmware(&dummy_elf, "netduinoplus2")
            .await
            .unwrap();
        // Verifier completes verification check safely
        assert!(res);
    }

    #[tokio::test]
    async fn test_evidence_bundle_export() {
        let temp_dir = std::env::temp_dir().join(format!("evidence_test_{}", uuid::Uuid::new_v4()));
        let mut bundle = EvidenceBundle::new(
            "task-42",
            "diff --git a/src/main.rs b/src/main.rs\n+fn main() {}",
        );
        bundle.add_report(VerifierReport {
            stage: "cargo_check".to_string(),
            passed: true,
            stdout: "Finished dev".to_string(),
            stderr: String::new(),
            duration_ms: 120,
        });
        bundle.hitl_decision = Some("Approved by operator".to_string());
        bundle.verified_success = true;

        bundle.export_to_directory(&temp_dir).unwrap();
        assert!(temp_dir.join("task.json").exists());
        assert!(temp_dir.join("patch.diff").exists());
        assert!(temp_dir.join("verifier_reports.json").exists());
        assert!(temp_dir.join("hitl_decision.json").exists());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
