pub mod execution;
pub mod landlock_sandbox;
pub mod podman_sandbox;
pub mod sandbox_macos;
pub mod sandbox_windows;
pub mod wasm_compiler;

pub use execution::{execute_in_sandbox, execute_wasm_sandbox, ExecutionResult};
pub use landlock_sandbox::LandlockSandbox;
pub use podman_sandbox::PodmanSandbox;
pub use sandbox_macos::MacosSandboxManager;
pub use sandbox_windows::WindowsSandboxManager;
pub use wasm_compiler::WasmCompiler;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_podman_build() {
        let tmp = tempdir().unwrap();
        let sandbox = PodmanSandbox::new(tmp.path().to_path_buf(), "redox-os/redox:latest");
        let (passed, stdout, _) = sandbox
            .run_build_command("echo 'Redox Toolchain Ready'")
            .await
            .unwrap();
        assert!(passed);
        assert!(stdout.contains("Redox Toolchain Ready"));
    }
}
