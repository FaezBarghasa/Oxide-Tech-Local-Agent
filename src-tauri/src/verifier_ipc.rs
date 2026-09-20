use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierRequest {
    #[serde(alias = "workspace_path")]
    pub workspace: String,
    pub task_id: Option<String>,
    pub export_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierReportDto {
    pub stage: String,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundleDto {
    pub task_id: String,
    pub timestamp: i64,
    pub git_diff: String,
    pub reports: Vec<VerifierReportDto>,
    pub verified_success: bool,
    pub hitl_decision: Option<String>,
    pub exported_path: Option<String>,
}

pub fn run_verification(req: VerifierRequest) -> anyhow::Result<EvidenceBundleDto> {
    let workspace = PathBuf::from(&req.workspace);
    let task_id = req
        .task_id
        .unwrap_or_else(|| format!("task-{}", uuid::Uuid::new_v4()));

    // Get current git diff
    let git_diff = match Command::new("git")
        .args(["diff", "HEAD"])
        .current_dir(&workspace)
        .output()
    {
        Ok(out) if out.status.success() => {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if s.trim().is_empty() {
                "// No unstaged git diff detected in workspace".to_string()
            } else {
                s
            }
        }
        _ => "// Git repository diff unavailable".to_string(),
    };

    let mut bundle = verifier::EvidenceBundle::new(&task_id, &git_diff);
    let mut reports_dto = Vec::new();

    // 1. Cargo Check stage
    let t0 = std::time::Instant::now();
    let cargo_status = Command::new("cargo")
        .arg("check")
        .arg("--workspace")
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

    let duration_ms = t0.elapsed().as_millis() as u64;

    bundle.add_report(verifier::VerifierReport {
        stage: "cargo_check".to_string(),
        passed,
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        duration_ms,
    });

    reports_dto.push(VerifierReportDto {
        stage: "cargo check --workspace".to_string(),
        passed,
        stdout,
        stderr,
        duration_ms,
    });

    bundle.verified_success = passed;
    bundle.hitl_decision = Some("Verified via Desktop Automated Suite".to_string());

    let mut exported_path = None;
    if let Some(target_dir_str) = req.export_path {
        let target_dir = PathBuf::from(&target_dir_str);
        std::fs::create_dir_all(&target_dir)?;
        bundle
            .export_to_directory(&target_dir)
            .context("Failed to export evidence bundle")?;
        exported_path = Some(target_dir.display().to_string());
    }

    Ok(EvidenceBundleDto {
        task_id,
        timestamp: bundle.timestamp.timestamp(),
        git_diff,
        reports: reports_dto,
        verified_success: bundle.verified_success,
        hitl_decision: bundle.hitl_decision,
        exported_path,
    })
}

#[tauri::command]
pub async fn verifier_run_suite(request: VerifierRequest) -> Result<EvidenceBundleDto, String> {
    tokio::task::spawn_blocking(move || run_verification(request))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn verifier_export_evidence(
    bundle_dto: EvidenceBundleDto,
    target_dir: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let dir = PathBuf::from(&target_dir);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        let mut bundle = verifier::EvidenceBundle::new(&bundle_dto.task_id, &bundle_dto.git_diff);
        for r in bundle_dto.reports {
            bundle.add_report(verifier::VerifierReport {
                stage: r.stage,
                passed: r.passed,
                stdout: r.stdout,
                stderr: r.stderr,
                duration_ms: r.duration_ms,
            });
        }
        bundle.verified_success = bundle_dto.verified_success;
        bundle.hitl_decision = bundle_dto.hitl_decision;

        bundle
            .export_to_directory(&dir)
            .map_err(|e| e.to_string())?;
        Ok(dir.display().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}