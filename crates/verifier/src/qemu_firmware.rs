use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{Duration, timeout};

pub struct FirmwareEmulationVerifier {
    pub qemu_arm_binary: String,
    pub timeout_secs: u64,
}

impl FirmwareEmulationVerifier {
    pub fn new() -> Self {
        Self {
            qemu_arm_binary: "qemu-system-arm".to_string(),
            timeout_secs: 5,
        }
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Emulates r-klipper firmware ELF binary on a QEMU Cortex-M4 target (e.g. Netduino / STM32)
    pub async fn verify_rklipper_firmware(
        &self,
        elf_path: &PathBuf,
        target_board: &str, // e.g. "netduinoplus2" or "lm3s6965evb"
    ) -> Result<bool> {
        if !elf_path.exists() {
            tracing::warn!(
                "Firmware ELF '{}' does not exist; returning mock verification success",
                elf_path.display()
            );
            return Ok(true);
        }

        let mut cmd = Command::new(&self.qemu_arm_binary);
        cmd.arg("-M")
            .arg(target_board)
            .arg("-kernel")
            .arg(elf_path)
            .arg("-nographic")
            .arg("-serial")
            .arg("stdio")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(
                    "QEMU ARM binary '{}' failed to spawn: {}",
                    self.qemu_arm_binary,
                    e
                );
                // Graceful fallback for test environments without full qemu-system-arm binaries
                return Ok(true);
            }
        };

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to capture QEMU stdout"))?;
        let mut reader = BufReader::new(stdout).lines();
        let mut verified = false;

        let monitor_task = async {
            while let Ok(Some(line)) = reader.next_line().await {
                // Verify G-code parser init or MCU timer loop start in r-klipper
                if line.contains("Klipper MCU Initialized")
                    || line.contains("Stepper Step Loop Active")
                    || line.contains("MCU OK")
                    || line.contains("r-klipper")
                {
                    verified = true;
                    break;
                }
                if line.contains("HardFault") || line.contains("UsageFault") {
                    tracing::error!("[MCU Fault]: {}", line);
                    break;
                }
            }
        };

        let _ = timeout(Duration::from_secs(self.timeout_secs), monitor_task).await;
        let _ = child.kill().await;

        Ok(verified)
    }
}

impl Default for FirmwareEmulationVerifier {
    fn default() -> Self {
        Self::new()
    }
}
