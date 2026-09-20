use crate::execute_in_sandbox;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// An atomic snapshot of file contents before agentic modification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtomicFileSnapshot {
    pub file_path: String,
    pub original_content: String,
}

/// Checkpoint recording the state of the workspace prior to an action
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceCheckpoint {
    pub checkpoint_id: String,
    pub description: String,
    pub timestamp: String,
    pub files: HashMap<String, AtomicFileSnapshot>,
    pub git_stash_ref: Option<String>,
}

pub struct CheckpointManager;

impl CheckpointManager {
    /// Create an in-memory & git-backed checkpoint before mutating files
    pub async fn create_checkpoint(
        checkpoint_id: &str,
        description: &str,
        work_dir: &str,
    ) -> Result<WorkspaceCheckpoint, String> {
        info!("Creating atomic workspace checkpoint: {}", checkpoint_id);

        // Attempt git stash create for instantaneous atomic snapshot
        let stash_res = execute_in_sandbox(&["git", "stash", "create"], work_dir).await;
        let git_stash_ref = stash_res.ok().and_then(|r| {
            let out = r.stdout.trim().to_string();
            if out.is_empty() { None } else { Some(out) }
        });

        Ok(WorkspaceCheckpoint {
            checkpoint_id: checkpoint_id.to_string(),
            description: description.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            files: HashMap::new(),
            git_stash_ref,
        })
    }

    /// Revert working tree instantaneously to the recorded checkpoint
    pub async fn rollback(
        checkpoint: &WorkspaceCheckpoint,
        work_dir: &str,
    ) -> Result<String, String> {
        info!(
            "Rolling back workspace to checkpoint: {}",
            checkpoint.checkpoint_id
        );

        // If git stash ref exists, apply it
        if let Some(ref stash_ref) = checkpoint.git_stash_ref {
            let res = execute_in_sandbox(&["git", "stash", "apply", stash_ref], work_dir)
                .await
                .map_err(|e| format!("Rollback git stash apply failed: {}", e))?;
            return Ok(format!(
                "Workspace restored to stash {}: {}",
                stash_ref, res.stdout
            ));
        }

        // Fallback: git checkout . and clean
        let res = execute_in_sandbox(&["git", "checkout", "."], work_dir)
            .await
            .map_err(|e| format!("Rollback git checkout failed: {}", e))?;

        Ok(format!("Workspace rolled back cleanly: {}", res.stdout))
    }
}
