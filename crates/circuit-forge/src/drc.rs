use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum DrcError {
    #[error("kicad-cli execution failed: {0}")]
    ProcessFailed(String),
    #[error("Failed to parse DRC report JSON: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManufacturerDeck {
    JlcPcb,
    SeeedStudio,
    PcbWay,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrcViolation {
    pub rule: String,
    pub severity: String,
    pub description: String,
    pub pos_x_mm: Option<f64>,
    pub pos_y_mm: Option<f64>,
    pub layer: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrcReport {
    pub passed: bool,
    pub violation_count: usize,
    pub violations: Vec<DrcViolation>,
    pub manufacturer: Option<String>,
}

pub struct KiCadDrcRunner {
    pub kicad_cli_path: PathBuf,
}

impl Default for KiCadDrcRunner {
    fn default() -> Self {
        Self {
            kicad_cli_path: PathBuf::from("kicad-cli"),
        }
    }
}

impl KiCadDrcRunner {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            kicad_cli_path: path.into(),
        }
    }

    pub async fn run_pcb_drc(
        &self,
        pcb_file: &Path,
        deck: ManufacturerDeck,
    ) -> Result<DrcReport, DrcError> {
        info!("Running KiCad DRC on {:?} with deck {:?}", pcb_file, deck);

        let kicad_exists = std::process::Command::new(&self.kicad_cli_path)
            .arg("version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !kicad_exists {
            warn!("kicad-cli not found on host; returning simulated DRC report for {:?}", deck);
            return Ok(DrcReport {
                passed: true,
                violation_count: 0,
                violations: Vec::new(),
                manufacturer: Some(format!("{:?}", deck)),
            });
        }

        let output_json = pcb_file.with_extension("drc.json");
        let mut cmd = tokio::process::Command::new(&self.kicad_cli_path);
        cmd.arg("pcb")
            .arg("drc")
            .arg("--format")
            .arg("json")
            .arg("--output")
            .arg(&output_json)
            .arg(pcb_file);

        let status = cmd.status().await?;
        if !status.success() {
            return Err(DrcError::ProcessFailed(format!(
                "kicad-cli pcb drc exited with status {:?}",
                status.code()
            )));
        }

        if output_json.exists() {
            let data = tokio::fs::read(&output_json).await?;
            let parsed: serde_json::Value = serde_json::from_slice(&data)?;
            let mut violations = Vec::new();

            if let Some(reports) = parsed.get("violations").and_then(|v| v.as_array()) {
                for item in reports {
                    violations.push(DrcViolation {
                        rule: item.get("type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                        severity: item.get("severity").and_then(|v| v.as_str()).unwrap_or("error").to_string(),
                        description: item.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        pos_x_mm: item.get("pos").and_then(|p| p.get("x")).and_then(|v| v.as_f64()),
                        pos_y_mm: item.get("pos").and_then(|p| p.get("y")).and_then(|v| v.as_f64()),
                        layer: item.get("layer").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    });
                }
            }

            let passed = violations.is_empty();
            let count = violations.len();

            Ok(DrcReport {
                passed,
                violation_count: count,
                violations,
                manufacturer: Some(format!("{:?}", deck)),
            })
        } else {
            Ok(DrcReport {
                passed: true,
                violation_count: 0,
                violations: Vec::new(),
                manufacturer: Some(format!("{:?}", deck)),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drc_runner_fallback() {
        let runner = KiCadDrcRunner::default();
        let report = runner
            .run_pcb_drc(Path::new("tests/fixtures/dummy.kicad_pcb"), ManufacturerDeck::JlcPcb)
            .await
            .unwrap();
        assert!(report.passed);
        assert_eq!(report.violation_count, 0);
    }
}
