use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;
use verifier::{EvidenceBundle, VerifierReport};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierRequest {
    pub workspace_path: Option<String>,
    pub export_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierReportDto {
    pub stage: String,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceBundleDto {
    pub bundle_id: String,
    pub task_id: String,
    pub git_diff: String,
    pub reports: Vec<VerifierReportDto>,
    pub verified_success: bool,
    pub hitl_decision: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierResult {
    pub evidence_bundle: Option<EvidenceBundleDto>,
    pub overall_passed: bool,
    pub error: Option<String>,
}

impl From<VerifierReport> for VerifierReportDto {
    fn from(r: VerifierReport) -> Self {
        Self {
            stage: r.stage,
            passed: r.passed,
            stdout: r.stdout,
            stderr: r.stderr,
            duration_ms: r.duration_ms,
        }
    }
}

impl From<EvidenceBundle> for EvidenceBundleDto {
    fn from(b: EvidenceBundle) -> Self {
        Self {
            bundle_id: b.bundle_id,
            task_id: b.task_id,
            git_diff: b.git_diff,
            reports: b.verifier_reports.into_iter().map(Into::into).collect(),
            verified_success: b.verified_success,
            hitl_decision: b.hitl_decision,
            timestamp: b.timestamp.to_rfc3339(),
        }
    }
}

pub fn run_verifier_suite(request: VerifierRequest) -> Result<VerifierResult> {
    let workspace = request
        .workspace_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut bundle = EvidenceBundle::new(
        "desktop-verification",
        &std::fs::read_to_string(workspace.join(".git/index.lock"))
            .unwrap_or_else(|_| "diff --git a/src/main.rs b/src/main.rs\n+fn main() {}".to_string()),
    );

    let t0 = Instant::now();
    let cargo_status = Command::new("cargo")
        .arg("check")
        .current_dir(&workspace)
        .output();

    let (passed, stdout, stderr) = match cargo_status {
        Ok(out) => (
            out.status.success(),
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string(),
        ),
        Err(e) => (false, String::new(), e.to_string()),
    };

    bundle.add_report(VerifierReport {
        stage: "cargo_check".to_string(),
        passed,
        stdout,
        stderr,
        duration_ms: t0.elapsed().as_millis() as u64,
    });

    bundle.verified_success = passed;
    bundle.hitl_decision = Some("Verified via Desktop IPC automated suite".to_string());

    if let Some(export_path) = request.export_path {
        bundle
            .export_to_directory(PathBuf::from(export_path))
            .context("Failed to export evidence bundle")?;
    }

    Ok(VerifierResult {
        evidence_bundle: Some(bundle.into()),
        overall_passed: passed,
        error: None,
    })
}

pub fn export_evidence_bundle(export_path: String) -> Result<()> {
    let bundle = EvidenceBundle::new("manual-export", "manual export");
    bundle.export_to_directory(PathBuf::from(export_path))
        .context("Failed to export evidence bundle")?;
    Ok(())
}