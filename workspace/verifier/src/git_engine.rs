use serde::{Deserialize, Serialize};
use tracing::info;
use crate::execute_in_sandbox;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitCommitInfo {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitStatusResult {
    pub branch: String,
    pub clean: bool,
    pub modified_files: Vec<String>,
    pub untracked_files: Vec<String>,
}

pub struct GitEngine;

impl GitEngine {
    /// Get current working branch and modified / untracked files
    pub async fn status(work_dir: &str) -> Result<GitStatusResult, String> {
        let res = execute_in_sandbox(&["git", "status", "--porcelain", "-b"], work_dir)
            .await
            .map_err(|e| format!("Failed to get git status: {}", e))?;

        let mut branch = "main".to_string();
        let mut modified_files = Vec::new();
        let mut untracked_files = Vec::new();

        for line in res.stdout.lines() {
            if line.starts_with("## ") {
                let b = line.trim_start_matches("## ").split("...").next().unwrap_or("main");
                branch = b.to_string();
            } else if line.starts_with("?? ") {
                untracked_files.push(line[3..].trim().to_string());
            } else if !line.trim().is_empty() {
                let f = line.get(3..).unwrap_or(line).trim().to_string();
                modified_files.push(f);
            }
        }

        let clean = modified_files.is_empty() && untracked_files.is_empty();

        Ok(GitStatusResult {
            branch,
            clean,
            modified_files,
            untracked_files,
        })
    }

    /// Create and checkout a new task branch
    pub async fn create_branch(work_dir: &str, branch_name: &str) -> Result<String, String> {
        info!("Creating and checking out git branch: {}", branch_name);
        let res = execute_in_sandbox(&["git", "checkout", "-b", branch_name], work_dir)
            .await
            .map_err(|e| format!("Failed to create branch: {}", e))?;

        if res.exit_code != 0 {
            // If already exists, checkout
            let _ = execute_in_sandbox(&["git", "checkout", branch_name], work_dir).await;
        }

        Ok(format!("Checked out branch: {}", branch_name))
    }

    /// Stage specific files or all changes
    pub async fn add(work_dir: &str, files: Option<&[&str]>) -> Result<String, String> {
        let mut cmd = vec!["git", "add"];
        if let Some(f_list) = files {
            cmd.extend(f_list);
        } else {
            cmd.push(".");
        }

        let res = execute_in_sandbox(&cmd, work_dir)
            .await
            .map_err(|e| format!("git add failed: {}", e))?;

        if res.exit_code == 0 {
            Ok("Files staged successfully".to_string())
        } else {
            Err(res.stderr)
        }
    }

    /// Author a semantic commit
    pub async fn commit(work_dir: &str, message: &str) -> Result<String, String> {
        let res = execute_in_sandbox(&["git", "commit", "-m", message], work_dir)
            .await
            .map_err(|e| format!("git commit failed: {}", e))?;

        if res.exit_code == 0 {
            Ok(res.stdout)
        } else {
            Err(res.stderr)
        }
    }

    /// Generate unified diff for workspace changes
    pub async fn diff(work_dir: &str) -> Result<String, String> {
        let res = execute_in_sandbox(&["git", "diff"], work_dir)
            .await
            .map_err(|e| format!("git diff failed: {}", e))?;

        Ok(res.stdout)
    }
}
