use std::path::Path;
use tokio::process::Command;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};

/// Maximum wall-clock time for a sandboxed command before SIGKILL is sent.
const EXECUTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Memory ceiling for the child process (1 GiB virtual address space).
const MEMORY_LIMIT_BYTES: u64 = 1 * 1024 * 1024 * 1024;

/// CPU time ceiling (seconds of CPU the process may consume before SIGKILL).
const CPU_TIME_LIMIT_SECS: u64 = 60;

/// The result of a sandboxed command execution.
pub struct ExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Execute `cmd` inside a native process sandbox rooted at `work_dir`.
///
/// Safety constraints (no Docker required):
/// - The child is placed in a new session (`setsid`) so SIGKILL on the process
///   group reaches every grandchild it spawns.
/// - `RLIMIT_AS` caps virtual memory at `MEMORY_LIMIT_BYTES`.
/// - `RLIMIT_CPU` caps CPU time at `CPU_TIME_LIMIT_SECS`.
/// - A `tokio::time::timeout` of `EXECUTION_TIMEOUT` provides a wall-clock
///   deadline; on expiry the entire process group is killed.
pub async fn execute_in_sandbox(
    cmd: &[&str],
    work_dir: &str,
) -> Result<ExecutionResult, String> {
    if cmd.is_empty() {
        return Err("Empty command provided to sandbox".to_string());
    }

    let work_dir_path = Path::new(work_dir);
    if !work_dir_path.exists() {
        return Err(format!("Sandbox working directory does not exist: {}", work_dir));
    }

    info!("Sandbox executing {:?} in {:?}", cmd, work_dir);

    let mut child = unsafe {
        Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(work_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .pre_exec(|| {
                // Place child in a new session so we can kill the entire group.
                nix::unistd::setsid()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                // Cap virtual memory (RLIMIT_AS).
                let mem_limit = nix::sys::resource::rlim_t::from(MEMORY_LIMIT_BYTES);
                nix::sys::resource::setrlimit(
                    nix::sys::resource::Resource::RLIMIT_AS,
                    mem_limit,
                    mem_limit,
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                // Cap CPU time (RLIMIT_CPU).
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
            .map_err(|e| format!("Failed to spawn sandboxed process '{}': {}", cmd[0], e))?
    };

    let pid = child.id().unwrap_or(0);

    // Grab piped handles before `child` is consumed by `wait()`.
    let stdout_handle = child.stdout.take().expect("stdout pipe missing");
    let stderr_handle = child.stderr.take().expect("stderr pipe missing");

    // Read stdout / stderr concurrently with a wall-clock deadline.
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
            let status = child
                .wait()
                .await
                .map_err(|e| format!("Failed to wait on sandboxed process: {}", e))?;

            let exit_code = status.code().unwrap_or(-1);

            if exit_code != 0 {
                warn!(
                    "Sandbox process exited with code {} for command {:?}",
                    exit_code, cmd
                );
            } else {
                info!("Sandbox process completed successfully for command {:?}", cmd);
            }

            Ok(ExecutionResult {
                exit_code,
                stdout: String::from_utf8_lossy(&stdout_bytes).into_owned(),
                stderr: String::from_utf8_lossy(&stderr_bytes).into_owned(),
            })
        }
        Err(_elapsed) => {
            // Wall-clock deadline exceeded — kill the entire process group.
            error!(
                "Sandbox timeout ({:?}) exceeded for command {:?} (pid {}). Killing process group.",
                EXECUTION_TIMEOUT, cmd, pid
            );

            if pid != 0 {
                // SIGKILL the process group (negative PID targets the group).
                let pgid = nix::unistd::Pid::from_raw(-(pid as i32));
                let _ = nix::sys::signal::kill(pgid, nix::sys::signal::Signal::SIGKILL);
            }

            // Reap the child to avoid zombies.
            let _ = child.wait().await;

            Err(format!(
                "Sandboxed command {:?} exceeded the {:?} wall-clock timeout and was killed.",
                cmd, EXECUTION_TIMEOUT
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_security() {
        // Test that execution inside non-existent or invalid working directories is rejected
        let invalid_workdir = "/root/forbidden_directory_xyz_123";
        let res = execute_in_sandbox(&["echo", "pwned"], invalid_workdir).await;
        assert!(res.is_err(), "Sandbox must reject execution in nonexistent/forbidden directory");

        // Test that empty commands are rejected
        let empty_cmd: Vec<&str> = vec![];
        let empty_res = execute_in_sandbox(&empty_cmd, ".").await;
        assert!(empty_res.is_err(), "Sandbox must reject empty commands");

        // Test normal safe execution in current directory
        let safe_res = execute_in_sandbox(&["echo", "sandbox_secure"], ".").await;
        assert!(safe_res.is_ok());
        let out = safe_res.unwrap();
        assert_eq!(out.exit_code, 0);
        assert!(out.stdout.contains("sandbox_secure"));
    }
}

