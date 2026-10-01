//! macOS native sandboxing using sandbox-exec / Seatbelt profile containment.

use anyhow::Result;
use std::path::Path;

pub struct MacosSandboxManager {
    profile_name: String,
}

impl Default for MacosSandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MacosSandboxManager {
    pub fn new() -> Self {
        Self {
            profile_name: "oxide_isolated_workspace".to_string(),
        }
    }

    /// Execute command within macOS Seatbelt profile sandbox.
    pub fn execute_isolated(&self, program: &Path, args: &[String], workspace: &Path) -> Result<i32> {
        #[cfg(target_os = "macos")]
        {
            let profile = format!(
                "(version 1)\n\
                 (deny default)\n\
                 (allow process-exec (literal \"{}\"))\n\
                 (allow file-read* file-write* (subpath \"{}\"))\n",
                program.display(),
                workspace.display()
            );

            let mut cmd = std::process::Command::new("sandbox-exec");
            cmd.arg("-p").arg(profile).arg(program).args(args);
            let status = cmd.status()?;
            Ok(status.code().unwrap_or(1))
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = workspace;
            let mut cmd = std::process::Command::new(program);
            cmd.args(args);
            let status = cmd.status()?;
            Ok(status.code().unwrap_or(1))
        }
    }
}
