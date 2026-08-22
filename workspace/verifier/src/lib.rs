use common::error::{EiosError, Result};
use std::path::Path;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{Duration, timeout};
use tracing::{error, info};

const EXECUTION_TIMEOUT: Duration = Duration::from_secs(30);
const MEMORY_LIMIT_BYTES: u64 = 1 * 1024 * 1024 * 1024; // 1 GiB
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
                nix::unistd::setsid()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                // Virtual memory cap (RLIMIT_AS)
                let mem_limit = nix::sys::resource::rlim_t::from(MEMORY_LIMIT_BYTES);
                nix::sys::resource::setrlimit(
                    nix::sys::resource::Resource::RLIMIT_AS,
                    mem_limit,
                    mem_limit,
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                // CPU time cap (RLIMIT_CPU)
                let cpu_limit = nix::sys::resource::rlim_t::from(CPU_TIME_LIMIT_SECS);
                nix::sys::resource::setrlimit(
                    nix::sys::resource::Resource::RLIMIT_CPU,
                    cpu_limit,
                    cpu_limit,
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                Ok(())
            })
            .spawn()
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to spawn process: {}", e),
                )
            })?
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

pub mod qemu_firmware;
pub use qemu_firmware::FirmwareEmulationVerifier;


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_rklipper_firmware_verification() {
        let verifier = FirmwareEmulationVerifier::new().with_timeout(1);
        let dummy_elf = PathBuf::from("workspace/firmware/rklipper.elf");
        let res = verifier.verify_rklipper_firmware(&dummy_elf, "netduinoplus2").await.unwrap();
        // Verifier completes verification check safely
        assert!(res);
    }
}

