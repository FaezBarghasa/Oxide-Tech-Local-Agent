use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Commit metadata representation in temporal memory
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporalCommit {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: String,
    pub files_changed: Vec<String>,
}

/// Churn score and evolutionary volatility metrics for a specific file/module
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleChurnMetrics {
    pub file_path: String,
    pub modification_count: usize,
    pub churn_risk_score: f32, // 0.0 to 1.0 (higher = higher volatility/regression risk)
    pub top_co_changed_files: Vec<(String, f32)>, // (other_file, coupling_percentage)
}

/// Temporal Git Engine: Maintains evolutionary history and co-change coupling
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TemporalGitMemory {
    pub commits: Vec<TemporalCommit>,
    pub file_edit_counts: HashMap<String, usize>,
    pub co_change_matrix: HashMap<String, HashMap<String, usize>>,
}

impl TemporalGitMemory {
    pub fn new() -> Self {
        Self {
            commits: Vec::new(),
            file_edit_counts: HashMap::new(),
            co_change_matrix: HashMap::new(),
        }
    }

    /// Record a historical or newly authored commit
    pub fn record_commit(&mut self, commit: TemporalCommit) {
        for file in &commit.files_changed {
            *self.file_edit_counts.entry(file.clone()).or_insert(0) += 1;
        }

        // Record pairwise co-change
        let files = &commit.files_changed;
        for i in 0..files.len() {
            for j in (i + 1)..files.len() {
                let f1 = &files[i];
                let f2 = &files[j];

                *self
                    .co_change_matrix
                    .entry(f1.clone())
                    .or_default()
                    .entry(f2.clone())
                    .or_insert(0) += 1;

                *self
                    .co_change_matrix
                    .entry(f2.clone())
                    .or_default()
                    .entry(f1.clone())
                    .or_insert(0) += 1;
            }
        }

        self.commits.push(commit);
    }

    /// Calculate churn metrics and co-change predictions for a file
    pub fn get_churn_metrics(&self, file_path: &str) -> ModuleChurnMetrics {
        let edits = self.file_edit_counts.get(file_path).copied().unwrap_or(0);
        let max_edits = self
            .file_edit_counts
            .values()
            .max()
            .copied()
            .unwrap_or(1)
            .max(1);

        let churn_risk_score = (edits as f32 / max_edits as f32).min(1.0);

        let mut top_co_changed = Vec::new();
        if let Some(co_map) = self.co_change_matrix.get(file_path) {
            for (other_file, count) in co_map {
                let coupling = *count as f32 / edits.max(1) as f32;
                top_co_changed.push((other_file.clone(), coupling));
            }
            top_co_changed
                .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            top_co_changed.truncate(5);
        }

        ModuleChurnMetrics {
            file_path: file_path.to_string(),
            modification_count: edits,
            churn_risk_score,
            top_co_changed_files: top_co_changed,
        }
    }
}
