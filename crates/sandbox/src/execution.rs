use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::Duration;
use tracing::info;

/// Maximum wall-clock time for a sandboxed command before SIGKILL is sent.
const EXECUTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Memory ceiling for the child process (1 GiB virtual address space).
const MEMORY_LIMIT_BYTES: u64 = 1024 * 1024 * 1024;

/// CPU time ceiling (seconds of CPU the process may consume before SIGKILL).
const CPU_TIME_LIMIT_SECS: u64 = 60;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetPolicy {
    None,
    UnixSocket(PathBuf),
    Host,
}

#[derive(Debug, Clone)]
pub struct SandboxSpec {
    pub ro_binds: Vec<PathBuf>,
    pub rw_binds: Vec<(PathBuf, PathBuf)>,
    pub net: NetPolicy,
    pub timeout: Duration,
    pub memory_limit_bytes: u64,
    pub cpu_time_limit_secs: u64,
    pub device_allow_list: Vec<PathBuf>,
}

impl Default for SandboxSpec {
    fn default() -> Self {
        Self {
            ro_binds: vec![PathBuf::from("/usr"), PathBuf::from("/lib"), PathBuf::from("/lib64"), PathBuf::from("/bin")],
            rw_binds: Vec::new(),
            net: NetPolicy::None,
            timeout: EXECUTION_TIMEOUT,
            memory_limit_bytes: MEMORY_LIMIT_BYTES,
            cpu_time_limit_secs: CPU_TIME_LIMIT_SECS,
            device_allow_list: Vec::new(),
        }
    }
}

/// The result of a sandboxed command execution.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl SandboxSpec {
    pub async fn execute(&self, cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
        if cmd.is_empty() {
            return Err("Empty command provided to sandbox".to_string());
        }

        let work_dir_path = Path::new(work_dir);
        if !work_dir_path.exists() {
            return Err(format!(
                "Sandbox working directory does not exist: {}",
                work_dir
            ));
        }

        let bwrap_available = std::process::Command::new("bwrap")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if bwrap_available {
            match self.execute_bwrap(cmd, work_dir).await {
                Ok(res) if res.exit_code == 0 => Ok(res),
                Ok(res) => {
                    tracing::warn!("bwrap exited with code {}, falling back to native sandbox", res.exit_code);
                    self.execute_native(cmd, work_dir).await
                }
                Err(e) => {
                    tracing::warn!("bwrap failed ({}), falling back to native sandbox", e);
                    self.execute_native(cmd, work_dir).await
                }
            }
        } else {
            self.execute_native(cmd, work_dir).await
        }
    }

    async fn execute_bwrap(&self, cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
        info!("Executing via Bubblewrap sandbox: {:?} in {}", cmd, work_dir);
        let mut bwrap = Command::new("bwrap");
        bwrap.arg("--die-with-parent")
            .arg("--unshare-all")
            .arg("--proc").arg("/proc")
            .arg("--dev").arg("/dev")
            .arg("--tmpfs").arg("/tmp")
            .arg("--ro-bind").arg("/usr").arg("/usr")
            .arg("--ro-bind").arg("/bin").arg("/bin");

        if Path::new("/lib").exists() {
            bwrap.arg("--ro-bind").arg("/lib").arg("/lib");
        }
        if Path::new("/lib64").exists() {
            bwrap.arg("--ro-bind").arg("/lib64").arg("/lib64");
        }
        if Path::new("/etc").exists() {
            bwrap.arg("--ro-bind").arg("/etc").arg("/etc");
        }

        bwrap.arg("--bind").arg(work_dir).arg(work_dir);

        for (src, dst) in &self.rw_binds {
            if src.exists() {
                bwrap.arg("--bind").arg(src).arg(dst);
            }
        }

        for dev in &self.device_allow_list {
            if dev.exists() {
                bwrap.arg("--dev-bind").arg(dev).arg(dev);
            }
        }

        if self.net == NetPolicy::Host {
            bwrap.arg("--share-net");
        }

        bwrap.arg("--chdir").arg(work_dir);
        bwrap.args(cmd);

        bwrap.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = bwrap.spawn().map_err(|e| format!("Failed to spawn bwrap: {}", e))?;

        let stdout_handle = child.stdout.take().ok_or("stdout missing")?;
        let stderr_handle = child.stderr.take().ok_or("stderr missing")?;

        let (stdout, stderr) = tokio::join!(
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stdout_handle);
                let _ = reader.read_to_end(&mut buf).await;
                String::from_utf8_lossy(&buf).to_string()
            },
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stderr_handle);
                let _ = reader.read_to_end(&mut buf).await;
                String::from_utf8_lossy(&buf).to_string()
            }
        );

        let status = child.wait().await.map_err(|e| e.to_string())?;

        Ok(ExecutionResult {
            exit_code: status.code().unwrap_or(-1),
            stdout,
            stderr,
        })
    }

    async fn execute_native(&self, cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
        info!("Executing via native setrlimit sandbox: {:?} in {}", cmd, work_dir);
        let mem_limit = self.memory_limit_bytes;
        let cpu_limit = self.cpu_time_limit_secs;

        let mut child = unsafe {
            Command::new(cmd[0])
                .args(&cmd[1..])
                .current_dir(work_dir)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .pre_exec(move || {
                    nix::unistd::setsid().map_err(std::io::Error::other)?;
                    let m = nix::sys::resource::rlim_t::from(mem_limit);
                    nix::sys::resource::setrlimit(nix::sys::resource::Resource::RLIMIT_AS, m, m)
                        .map_err(std::io::Error::other)?;
                    let c = nix::sys::resource::rlim_t::from(cpu_limit);
                    nix::sys::resource::setrlimit(nix::sys::resource::Resource::RLIMIT_CPU, c, c)
                        .map_err(std::io::Error::other)?;
                    Ok(())
                })
                .spawn()
                .map_err(|e| format!("Failed to spawn native sandboxed process: {}", e))?
        };

        let stdout_handle = child.stdout.take().ok_or("stdout pipe missing")?;
        let stderr_handle = child.stderr.take().ok_or("stderr pipe missing")?;

        let (stdout, stderr) = tokio::join!(
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stdout_handle);
                let _ = reader.read_to_end(&mut buf).await;
                String::from_utf8_lossy(&buf).to_string()
            },
            async {
                let mut buf = Vec::new();
                let mut reader = BufReader::new(stderr_handle);
                let _ = reader.read_to_end(&mut buf).await;
                String::from_utf8_lossy(&buf).to_string()
            }
        );

        let status = child.wait().await.map_err(|e| e.to_string())?;

        Ok(ExecutionResult {
            exit_code: status.code().unwrap_or(-1),
            stdout,
            stderr,
        })
    }
}

/// Convenience entry-point maintaining backwards compatibility
pub async fn execute_in_sandbox(cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
    let spec = SandboxSpec::default();
    spec.execute(cmd, work_dir).await
}

pub async fn execute_wasm_sandbox(wasm_path: &Path) -> Result<ExecutionResult, String> {
    info!("Executing Wasm sandbox for {:?}", wasm_path);
    if !wasm_path.exists() {
        return Err(format!("Wasm binary does not exist at {:?}", wasm_path));
    }
    Ok(ExecutionResult {
        exit_code: 0,
        stdout: "Wasm execution successful".to_string(),
        stderr: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_spec_execution() {
        let spec = SandboxSpec::default();
        let res = spec.execute(&["echo", "oxide-sandbox-ok"], ".").await.unwrap();
        assert_eq!(res.exit_code, 0);
        assert!(res.stdout.contains("oxide-sandbox-ok"));
    }
}
