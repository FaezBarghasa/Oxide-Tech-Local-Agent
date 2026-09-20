use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierReport {
    pub stage: String,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: String,
    pub task_id: String,
    pub timestamp: DateTime<Utc>,
    pub git_diff: String,
    pub verifier_reports: Vec<VerifierReport>,
    pub hitl_decision: Option<String>,
    pub verified_success: bool,
}

impl EvidenceBundle {
    pub fn new(task_id: impl Into<String>, git_diff: impl Into<String>) -> Self {
        Self {
            bundle_id: uuid::Uuid::new_v4().to_string(),
            task_id: task_id.into(),
            timestamp: Utc::now(),
            git_diff: git_diff.into(),
            verifier_reports: Vec::new(),
            hitl_decision: None,
            verified_success: false,
        }
    }

    pub fn add_report(&mut self, report: VerifierReport) {
        if !report.passed {
            self.verified_success = false;
        }
        self.verifier_reports.push(report);
    }

    /// Exports the bundle to a structured directory
    pub fn export_to_directory(&self, output_dir: impl AsRef<Path>) -> Result<()> {
        let dir = output_dir.as_ref();
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create evidence bundle directory at {:?}", dir))?;

        // 1. Write task & bundle metadata
        let metadata_json = serde_json::to_string_pretty(self)?;
        std::fs::write(dir.join("task.json"), metadata_json)?;

        // 2. Write patch diff
        std::fs::write(dir.join("patch.diff"), &self.git_diff)?;

        // 3. Write verifier reports
        let reports_json = serde_json::to_string_pretty(&self.verifier_reports)?;
        std::fs::write(dir.join("verifier_reports.json"), reports_json)?;

        // 4. Write HITL decision if present
        if let Some(decision) = &self.hitl_decision {
            std::fs::write(dir.join("hitl_decision.json"), decision)?;
        }

        Ok(())
    }
}
