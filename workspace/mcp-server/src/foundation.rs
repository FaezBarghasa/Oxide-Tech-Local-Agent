use std::path::{Path, PathBuf};
use verifier::execute_in_sandbox;

/// Runs a cargo command in the given path (or workspace root if None) using the verifier sandbox.
pub async fn run_cargo(
    cmd_args: &[&str],
    workspace_path: Option<String>,
    root: &Path,
) -> Result<String, String> {
    let target_dir = workspace_path
        .map(|w| root.join(w))
        .unwrap_or_else(|| root.to_path_buf());
    let dir_str = target_dir.to_string_lossy().to_string();

    let mut full_cmd = vec!["cargo"];
    full_cmd.extend_from_slice(cmd_args);

    match execute_in_sandbox(&full_cmd, &dir_str).await {
        Ok(res) => {
            let output = format!(
                "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                res.exit_code, res.stdout, res.stderr
            );
            if res.exit_code == 0 {
                Ok(output)
            } else {
                Err(output)
            }
        }
        Err(e) => Err(format!("Cargo sandbox execution failed: {}", e)),
    }
}

/// Runs a git command in the given path (or workspace root if None) using the verifier sandbox.
pub async fn run_git(
    cmd_args: &[&str],
    repo_path: Option<String>,
    root: &Path,
) -> Result<String, String> {
    let target_dir = repo_path
        .map(|r| root.join(r))
        .unwrap_or_else(|| root.to_path_buf());
    let dir_str = target_dir.to_string_lossy().to_string();

    let mut full_cmd = vec!["git"];
    full_cmd.extend_from_slice(cmd_args);

    match execute_in_sandbox(&full_cmd, &dir_str).await {
        Ok(res) => {
            let output = format!(
                "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                res.exit_code, res.stdout, res.stderr
            );
            if res.exit_code == 0 {
                Ok(output)
            } else {
                Err(output)
            }
        }
        Err(e) => Err(format!("Git sandbox execution failed: {}", e)),
    }
}

/// Reads a file from the workspace root.
pub async fn fs_read(path: &str, root: &Path) -> Result<String, String> {
    let full_path = root.join(path);
    tokio::fs::read_to_string(&full_path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path, e))
}

/// Writes content to a file in the workspace root.
pub async fn fs_write(path: &str, content: &str, root: &Path) -> Result<String, String> {
    let full_path = root.join(path);
    if let Some(parent) = full_path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    tokio::fs::write(&full_path, content)
        .await
        .map(|_| format!("File written successfully to '{}'", path))
        .map_err(|e| format!("Failed to write file '{}': {}", path, e))
}

/// Moves/renames a file/directory in the workspace root.
pub async fn fs_move(src: &str, dest: &str, root: &Path) -> Result<String, String> {
    let full_src = root.join(src);
    let full_dest = root.join(dest);
    if let Some(parent) = full_dest.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    tokio::fs::rename(&full_src, &full_dest)
        .await
        .map(|_| format!("Moved '{}' to '{}' successfully", src, dest))
        .map_err(|e| format!("Failed to move file: {}", e))
}

/// Deletes a file in the workspace root.
pub async fn fs_delete(path: &str, root: &Path) -> Result<String, String> {
    let full_path = root.join(path);
    tokio::fs::remove_file(&full_path)
        .await
        .map(|_| format!("Deleted file '{}' successfully", path))
        .map_err(|e| format!("Failed to delete file '{}': {}", path, e))
}

/// Searches recursively for files matching a pattern in a directory.
pub async fn fs_search(
    query: &str,
    sub_dir: Option<String>,
    root: &Path,
) -> Result<String, String> {
    let search_dir = sub_dir
        .map(|d| root.join(d))
        .unwrap_or_else(|| root.to_path_buf());

    let mut results = Vec::new();
    let mut dirs_to_visit = vec![search_dir];

    while let Some(dir) = dirs_to_visit.pop() {
        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| format!("Failed to read directory '{:?}': {}", dir, e))?;

        while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
            let path = entry.path();
            if path.is_dir() {
                dirs_to_visit.push(path);
            } else if path.is_file() {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                if filename.contains(query) {
                    if let Ok(rel) = path.strip_prefix(root) {
                        results.push(rel.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    if results.is_empty() {
        Ok("No matching files found".to_string())
    } else {
        Ok(format!("Matching files found:\n{}", results.join("\n")))
    }
}

/// Evaluates a glob pattern in the workspace root.
pub async fn fs_glob(pattern: &str, root: &Path) -> Result<String, String> {
    let full_pattern = root.join(pattern).to_string_lossy().to_string();
    let mut results = Vec::new();

    // Fallback to simple matching using glob or directory walk if the standard library is preferred.
    // Let's do a basic walk and match for simplicity and robustness.
    let path_pat = PathBuf::from(&full_pattern);
    let name_pattern = path_pat
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let regex_pat = name_pattern.replace('*', ".*").replace('?', ".");
    let re = regex::Regex::new(&format!("^{}$", regex_pat))
        .unwrap_or_else(|_| regex::Regex::new(".*").unwrap());

    let search_dir = path_pat.parent().unwrap_or(root);
    if let Ok(mut entries) = tokio::fs::read_dir(search_dir).await {
        while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                if re.is_match(&filename) {
                    if let Ok(rel) = path.strip_prefix(root) {
                        results.push(rel.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    if results.is_empty() {
        Ok("No files matched the glob pattern".to_string())
    } else {
        Ok(format!("Matched files:\n{}", results.join("\n")))
    }
}

/// Runs a terminal command inside the sandbox.
pub async fn terminal_run(
    command: &str,
    args: &[String],
    work_dir: Option<String>,
    root: &Path,
) -> Result<String, String> {
    let target_dir = work_dir
        .map(|w| root.join(w))
        .unwrap_or_else(|| root.to_path_buf());
    let dir_str = target_dir.to_string_lossy().to_string();

    let mut full_cmd = vec![command];
    for arg in args {
        full_cmd.push(arg.as_str());
    }

    match execute_in_sandbox(&full_cmd, &dir_str).await {
        Ok(res) => {
            let output = format!(
                "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                res.exit_code, res.stdout, res.stderr
            );
            if res.exit_code == 0 {
                Ok(output)
            } else {
                Err(output)
            }
        }
        Err(e) => Err(format!("Terminal sandbox execution failed: {}", e)),
    }
}
