use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub name: String,
    pub command: String,
    pub required: bool,
    pub passed: bool,
    pub version: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorResult {
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
    pub checks: Vec<DiagnosticCheck>,
}

pub fn check_system_tool(name: &str, cmd: &str, required: bool) -> DiagnosticCheck {
    match Command::new(cmd).arg("--version").output() {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("installed")
                .trim()
                .to_string();
            DiagnosticCheck {
                name: name.to_string(),
                command: cmd.to_string(),
                required,
                passed: true,
                version: ver,
                error: None,
            }
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
            DiagnosticCheck {
                name: name.to_string(),
                command: cmd.to_string(),
                required,
                passed: false,
                version: "error".to_string(),
                error: Some(err),
            }
        }
        Err(e) => DiagnosticCheck {
            name: name.to_string(),
            command: cmd.to_string(),
            required,
            passed: false,
            version: "not found".to_string(),
            error: Some(e.to_string()),
        },
    }
}

pub fn run_diagnostics_scan() -> DoctorResult {
    let mut checks = vec![
        check_system_tool("Rust Compiler", "rustc", true),
        check_system_tool("Cargo", "cargo", true),
        check_system_tool("Node.js", "node", true),
        check_system_tool("pnpm", "pnpm", false),
        check_system_tool("Bubblewrap Sandbox", "bwrap", true),
        check_system_tool("Git", "git", true),
        check_system_tool("probe-rs (STM32/ARM)", "probe-rs", false),
        check_system_tool("QEMU x86_64", "qemu-system-x86_64", false),
        check_system_tool("KiCad CLI (EDA)", "kicad-cli", false),
        check_system_tool("Ngspice (Electrical Sim)", "ngspice", false),
        check_system_tool("Ollama (Local LLM)", "ollama", false),
    ];

    let gpu_check = match Command::new("nvidia-smi")
        .args(["--query-gpu=name,memory.total", "--format=csv,noheader"])
        .output()
    {
        Ok(out) if out.status.success() => {
            let gpu = String::from_utf8_lossy(&out.stdout).trim().to_string();
            DiagnosticCheck {
                name: "NVIDIA GPU Hardware Acceleration".to_string(),
                command: "nvidia-smi".to_string(),
                required: false,
                passed: true,
                version: gpu,
                error: None,
            }
        }
        _ => DiagnosticCheck {
            name: "NVIDIA GPU Hardware Acceleration".to_string(),
            command: "nvidia-smi".to_string(),
            required: false,
            passed: false,
            version: "No GPU / CPU Lite Mode".to_string(),
            error: Some("Operating in CPU Lite Mode".to_string()),
        },
    };
    checks.push(gpu_check);

    let udev_present = Path::new("/etc/udev/rules.d/99-probe-rs.rules").exists()
        || Path::new("/usr/lib/udev/rules.d/69-probe-rs.rules").exists();
    checks.push(DiagnosticCheck {
        name: "Hardware Debugger udev Rules".to_string(),
        command: "/etc/udev/rules.d/99-probe-rs.rules".to_string(),
        required: false,
        passed: udev_present,
        version: if udev_present {
            "present".to_string()
        } else {
            "missing".to_string()
        },
        error: if udev_present {
            None
        } else {
            Some("Run udev setup for non-root ST-Link/J-Link access".to_string())
        },
    });

    let mut passed = 0;
    let mut warnings = 0;
    let mut failed = 0;

    for c in &checks {
        if c.passed {
            passed += 1;
        } else if c.required {
            failed += 1;
        } else {
            warnings += 1;
        }
    }

    DoctorResult {
        passed,
        warnings,
        failed,
        checks,
    }
}

pub fn install_udev_rules() -> Result<String, String> {
    let script_path = Path::new("scripts/install_udev_rules.sh");
    let cmd = if script_path.exists() {
        format!("bash {}", script_path.display())
    } else {
        "curl -s https://probe.rs/files/69-probe-rs.rules | tee /etc/udev/rules.d/99-probe-rs.rules && udevadm control --reload && udevadm trigger".to_string()
    };

    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .map_err(|e| format!("Failed to execute pkexec: {}", e))?;

    if output.status.success() {
        Ok("Successfully installed probe-rs udev rules and reloaded daemon.".to_string())
    } else {
        Err(format!(
            "Failed with status: {}. {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[tauri::command]
pub async fn doctor_run_diagnostics() -> Result<DoctorResult, String> {
    tokio::task::spawn_blocking(run_diagnostics_scan)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn doctor_install_udev_rules() -> Result<String, String> {
    tokio::task::spawn_blocking(install_udev_rules)
        .await
        .map_err(|e| e.to_string())?
}