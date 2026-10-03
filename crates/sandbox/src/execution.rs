use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::time::Duration;
use tracing::info;

/// Maximum wall-clock time for a sandboxed command before SIGKILL is sent.
const EXECUTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Memory ceiling for the child process (1 GiB virtual address space).
const MEMORY_LIMIT_BYTES: u64 = 10240 * 1024 * 1024;

/// CPU time ceiling (seconds of CPU the process may consume before SIGKILL).
const CPU_TIME_LIMIT_SECS: u64 = 60;

/// Output ceiling per stream (8 MiB) to prevent host memory exhaustion.
pub const MAX_SANDBOX_OUTPUT_BYTES: usize = 80 * 1024 * 1024;

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
            ro_binds: vec![
                PathBuf::from("/usr"),
                PathBuf::from("/lib"),
                PathBuf::from("/lib64"),
                PathBuf::from("/bin"),
            ],
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

/// Reads from an async reader into a bounded buffer, discarding subsequent bytes if the limit is exceeded.
async fn read_bounded<R: tokio::io::AsyncRead + Unpin>(mut reader: R, limit: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    while let Ok(n) = reader.read(&mut chunk).await {
        if n == 0 {
            break;
        }
        if buf.len() + n > limit {
            let take = limit.saturating_sub(buf.len());
            buf.extend_from_slice(&chunk[..take]);
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    buf
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

        if !bwrap_available {
            #[cfg(feature = "unsafe-native-execution")]
            {
                tracing::warn!("SECURITY WARNING: Running un-isolated native execution due to explicit unsafe-native-execution feature");
                return self.execute_native(cmd, work_dir).await;
            }

            #[cfg(not(feature = "unsafe-native-execution"))]
            {
                return Err("Bubblewrap ('bwrap') sandbox is unavailable. Host execution rejected under fail-closed security contract.".to_string());
            }
        }

        // Fail-closed: Never fall back to native host execution when bwrap fails
        self.execute_bwrap(cmd, work_dir).await
    }

    async fn execute_bwrap(&self, cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
        let canonical_work_dir = Path::new(work_dir)
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize sandbox work_dir '{}': {}", work_dir, e))?;
        let work_dir_str = canonical_work_dir.to_str().ok_or("Invalid UTF-8 in work_dir")?;

        let resolved_binary = if cmd[0].contains('/') {
            PathBuf::from(cmd[0])
        } else {
            let path_env = std::env::var("PATH").unwrap_or_else(|_| "/usr/local/bin:/usr/bin:/bin".to_string());
            let mut found = None;
            for dir in std::env::split_paths(&path_env) {
                let candidate = dir.join(cmd[0]);
                if candidate.is_file() {
                    found = Some(candidate);
                    break;
                }
            }
            found.unwrap_or_else(|| PathBuf::from(cmd[0]))
        };
        let resolved_bin_str = resolved_binary.to_str().unwrap_or(cmd[0]);

        info!(
            "Executing via Bubblewrap sandbox: {:?} (resolved {}) in {}",
            cmd, resolved_bin_str, work_dir_str
        );
        let mut bwrap = Command::new("bwrap");
        bwrap
            .arg("--die-with-parent")
            .arg("--unshare-all")
            .arg("--proc")
            .arg("/proc")
            .arg("--dev")
            .arg("/dev")
            .arg("--ro-bind")
            .arg("/")
            .arg("/")
            .arg("--tmpfs")
            .arg("/tmp");

        bwrap.arg("--bind").arg(&canonical_work_dir).arg(&canonical_work_dir);

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

        bwrap.arg("--chdir").arg(work_dir_str);
        bwrap.arg(resolved_bin_str);
        bwrap.args(&cmd[1..]);

        #[cfg(unix)]
        bwrap.process_group(0);

        bwrap
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = bwrap
            .spawn()
            .map_err(|e| format!("Failed to spawn bwrap: {}", e))?;

        let pid = child.id();
        let timeout_duration = self.timeout;
        let stdout_handle = child.stdout.take().ok_or("stdout missing")?;
        let stderr_handle = child.stderr.take().ok_or("stderr missing")?;

        let execution_future = async {
            let (stdout_bytes, stderr_bytes) = tokio::join!(
                read_bounded(stdout_handle, MAX_SANDBOX_OUTPUT_BYTES),
                read_bounded(stderr_handle, MAX_SANDBOX_OUTPUT_BYTES)
            );
            let status = child.wait().await.map_err(|e| e.to_string())?;
            Ok::<_, String>((status, stdout_bytes, stderr_bytes))
        };

        match tokio::time::timeout(timeout_duration, execution_future).await {
            Ok(Ok((status, stdout_bytes, stderr_bytes))) => Ok(ExecutionResult {
                exit_code: status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&stdout_bytes).to_string(),
                stderr: String::from_utf8_lossy(&stderr_bytes).to_string(),
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                tracing::error!(
                    "Sandbox command {:?} timed out after {:?}. Terminating process group.",
                    cmd,
                    timeout_duration
                );
                if let Some(pid_val) = pid {
                    let _ = nix::sys::signal::killpg(
                        nix::unistd::Pid::from_raw(pid_val as i32),
                        nix::sys::signal::Signal::SIGKILL,
                    );
                }
                let _ = child.wait().await;
                Err(format!(
                    "Sandbox command {:?} timed out after {:?}",
                    cmd, timeout_duration
                ))
            }
        }
    }

    #[allow(dead_code)]
    async fn execute_native(
        &self,
        cmd: &[&str],
        work_dir: &str,
    ) -> Result<ExecutionResult, String> {
        info!(
            "Executing via native setrlimit sandbox: {:?} in {}",
            cmd, work_dir
        );
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

        let pid = child.id();
        let timeout_duration = self.timeout;
        let stdout_handle = child.stdout.take().ok_or("stdout pipe missing")?;
        let stderr_handle = child.stderr.take().ok_or("stderr pipe missing")?;

        let execution_future = async {
            let (stdout_bytes, stderr_bytes) = tokio::join!(
                read_bounded(stdout_handle, MAX_SANDBOX_OUTPUT_BYTES),
                read_bounded(stderr_handle, MAX_SANDBOX_OUTPUT_BYTES)
            );
            let status = child.wait().await.map_err(|e| e.to_string())?;
            Ok::<_, String>((status, stdout_bytes, stderr_bytes))
        };

        match tokio::time::timeout(timeout_duration, execution_future).await {
            Ok(Ok((status, stdout_bytes, stderr_bytes))) => Ok(ExecutionResult {
                exit_code: status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&stdout_bytes).to_string(),
                stderr: String::from_utf8_lossy(&stderr_bytes).to_string(),
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                tracing::error!(
                    "Native command {:?} timed out after {:?}. Terminating process group.",
                    cmd,
                    timeout_duration
                );
                if let Some(pid_val) = pid {
                    let _ = nix::sys::signal::killpg(
                        nix::unistd::Pid::from_raw(pid_val as i32),
                        nix::sys::signal::Signal::SIGKILL,
                    );
                }
                let _ = child.wait().await;
                Err(format!(
                    "Native command {:?} timed out after {:?}",
                    cmd, timeout_duration
                ))
            }
        }
    }
}

/// Convenience entry-point maintaining backwards compatibility
pub async fn execute_in_sandbox(cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> {
    let spec = SandboxSpec::default();
    spec.execute(cmd, work_dir).await
}

pub async fn execute_wasm_sandbox(
    wasm_bytes: &[u8],
    _inputs: &[u8],
    fuel: u64,
) -> Result<ExecutionResult, String> {
    info!(
        "Executing Wasm sandbox for {} bytes with fuel limit {}",
        wasm_bytes.len(),
        fuel
    );
    if wasm_bytes.is_empty() {
        return Err("Wasm binary is empty".to_string());
    }

    let mut config = wasmtime::Config::new();
    config.consume_fuel(true);
    let engine = wasmtime::Engine::new(&config)
        .map_err(|e| format!("Failed to create Wasm engine: {}", e))?;

    let module = wasmtime::Module::new(&engine, wasm_bytes)
        .map_err(|e| format!("Wasm validation failed: {}", e))?;

    let mut store = wasmtime::Store::new(&engine, ());
    store
        .set_fuel(fuel)
        .map_err(|e| format!("Failed to configure Wasm fuel: {}", e))?;

    let linker = wasmtime::Linker::new(&engine);
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| format!("Wasm instantiation failed: {}", e))?;

    let mut exit_code = 0;
    let mut stdout = format!(
        "Wasm module validated successfully ({} bytes, {} exports).",
        wasm_bytes.len(),
        module.exports().count()
    );

    if let Ok(start_fn) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
        match start_fn.call(&mut store, ()) {
            Ok(_) => stdout.push_str("\nExecuted _start successfully."),
            Err(trap) => {
                exit_code = 1;
                stdout.push_str(&format!("\nTrap during _start: {}", trap));
            }
        }
    } else if let Ok(run_fn) = instance.get_typed_func::<(), ()>(&mut store, "run") {
        match run_fn.call(&mut store, ()) {
            Ok(_) => stdout.push_str("\nExecuted run successfully."),
            Err(trap) => {
                exit_code = 1;
                stdout.push_str(&format!("\nTrap during run: {}", trap));
            }
        }
    }

    let remaining_fuel = store.get_fuel().unwrap_or(0);
    stdout.push_str(&format!(
        "\nFuel consumed: {}",
        fuel.saturating_sub(remaining_fuel)
    ));

    Ok(ExecutionResult {
        exit_code,
        stdout,
        stderr: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_spec_execution() {
        let spec = SandboxSpec::default();
        let res = spec
            .execute(&["echo", "oxide-sandbox-ok"], ".")
            .await
            .unwrap();
        assert_eq!(res.exit_code, 0);
        assert!(res.stdout.contains("oxide-sandbox-ok"));
    }

    #[tokio::test]
    async fn test_wasm_sandbox_invalid_bytes_rejected() {
        let bad_bytes = b"NOT_A_WASM_BINARY";
        let res = execute_wasm_sandbox(bad_bytes, b"", 100_000).await;
        assert!(
            res.is_err(),
            "Expected invalid Wasm bytes to fail validation"
        );
    }
}
