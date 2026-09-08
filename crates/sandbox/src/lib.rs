pub mod docker_sandbox;
pub mod execution;
pub mod wasm_compiler;

pub use docker_sandbox::DockerSandbox;
pub use execution::{execute_in_sandbox, execute_wasm_sandbox, ExecutionResult};
pub use wasm_compiler::WasmCompiler;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_docker_build() {
        let tmp = tempdir().unwrap();
        let sandbox = DockerSandbox::new(tmp.path().to_path_buf(), "redox-os/redox:latest");
        let (passed, stdout, _) = sandbox
            .run_build_command("echo 'Redox Toolchain Ready'")
            .await
            .unwrap();
        assert!(passed);
        assert!(stdout.contains("Redox Toolchain Ready"));
    }
}
