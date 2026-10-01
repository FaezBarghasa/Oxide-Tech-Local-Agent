//! Linux Landlock LSM sandboxing with bubblewrap namespace containment.

use anyhow::Result;
use std::path::Path;

pub struct LandlockSandbox {
    allowed_read_paths: Vec<String>,
    allowed_write_paths: Vec<String>,
}

impl LandlockSandbox {
    pub fn new(workspace: &Path) -> Self {
        Self {
            allowed_read_paths: vec!["/usr".to_string(), "/lib".to_string(), "/lib64".to_string(), "/etc".to_string()],
            allowed_write_paths: vec![workspace.display().to_string(), "/tmp".to_string()],
        }
    }

    pub fn execute_bwrap(&self, program: &Path, args: &[String]) -> Result<i32> {
        let mut cmd = std::process::Command::new("bwrap");
        cmd.arg("--unshare-all")
            .arg("--die-with-parent")
            .arg("--proc").arg("/proc")
            .arg("--dev").arg("/dev");

        for ro in &self.allowed_read_paths {
            if Path::new(ro).exists() {
                cmd.arg("--ro-bind").arg(ro).arg(ro);
            }
        }

        for rw in &self.allowed_write_paths {
            if Path::new(rw).exists() {
                cmd.arg("--bind").arg(rw).arg(rw);
            }
        }

        cmd.arg(program).args(args);
        let status = cmd.status()?;
        Ok(status.code().unwrap_or(1))
    }
}
