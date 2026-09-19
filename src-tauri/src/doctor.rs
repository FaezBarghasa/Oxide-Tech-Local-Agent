use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticCheck {
    pub name: String,
    pub command: String,
    pub required: bool,
    pub passed: bool,
    pub version: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorResult {
    pub checks: Vec<DiagnosticCheck>,
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
    pub ready: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UdevInstallResult {
    pub success: bool,
    pub message: String,
}

fn check_system_tool(name: &str, cmd: &str, required: bool) -> DiagnosticCheck {
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

pub fn run_diagnostics() -> Result<DoctorResult> {
    info!("Running comprehensive system diagnostics");

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
        version: if udev_present { "present" } else { "missing" }.to_string(),
        error: if udev_present {
            None
        } else {
            Some("Run udev setup for non-root ST-Link/J-Link access".to_string())
        },
    });

    let embed_check = match crate::memory::resolve_embed_bin() {
        Ok(p) => DiagnosticCheck {
            name: "Oxide-Embed STAIR Context Engine".to_string(),
            command: p.display().to_string(),
            required: false,
            passed: true,
            version: "bundled sidecar ready".to_string(),
            error: None,
        },
        Err(e) => DiagnosticCheck {
            name: "Oxide-Embed STAIR Context Engine".to_string(),
            command: "oxide-embed".to_string(),
            required: false,
            passed: false,
            version: "missing".to_string(),
            error: Some(e),
        },
    };
    checks.push(embed_check);

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

    let ready = failed == 0;

    Ok(DoctorResult {
        checks,
        passed,
        warnings,
        failed,
        ready,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

pub fn install_udev_rules() -> Result<UdevInstallResult> {
    info!("Installing udev rules for hardware debuggers");

    let rules_content = r#"# probe-rs udev rules for ST-Link, J-Link, CMSIS-DAP
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="374[48bc]", MODE="0666", GROUP="plugdev", TAG+="uaccess"
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="374b", MODE="0666", GROUP="plugdev", TAG+="uaccess"
SUBSYSTEM=="usb", ATTR{idVendor}=="1366", ATTR{idProduct}=="010[1-5]", MODE="0666", GROUP="plugdev", TAG+="uaccess"
SUBSYSTEM=="usb", ATTR{idVendor}=="2b73", ATTR{idProduct}=="0[0-9a-f]{3}", MODE="0666", GROUP="plugdev", TAG+="uaccess"
"#;

    let dest = Path::new("/etc/udev/rules.d/99-probe-rs.rules");

    let pkexec_result = Command::new("pkexec")
        .args([
            "tee",
            dest.to_str().unwrap(),
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    match pkexec_result {
        Ok(mut child) => {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(rules_content.as_bytes())?;
            }
            let output = child.wait()?;
            if output.success() {
                let _ = Command::new("udevadm").args(["control", "--reload-rules"]).status();
                let _ = Command::new("udevadm").args(["trigger"]).status();
                Ok(UdevInstallResult {
                    success: true,
                    message: "udev rules installed successfully. Unplug and replug your debugger.".to_string(),
                })
            } else {
                Ok(UdevInstallResult {
                    success: false,
                    message: "pkexec failed (user cancelled or policy denied)".to_string(),
                })
            }
        }
        Err(e) => Ok(UdevInstallResult {
            success: false,
            message: format!("Failed to invoke pkexec: {e}. Is polkit installed?"),
        }),
    }
}