//! oxide-embed memory bridge.
//!
//! All project memory flows through the `oxide-embed` binary:
//!   init / index / search / context / remember / recall / conflicts / explain.
//!
//! Binary resolution order (first hit wins):
//!   1. `OXIDE_EMBED_BIN` env var (explicit override, used by dev shells)
//!   2. `<current-exe-dir>/oxide-embed` (Tauri `externalBin` sidecar staging)
//!   3. `/usr/lib/oxide-agent/oxide-embed` (deb install layout)
//!   4. `/usr/bin/oxide-embed`
//!   5. `oxide-embed` on `PATH` (dev default, e.g. `~/.local/bin/oxide-embed`)

use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct MemoryEnv {
    pub bin: String,
    pub version: String,
    pub cwd: String,
    pub manifest_present: bool,
}

#[derive(Debug, Serialize)]
pub struct EmbedResult {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(p) = std::env::var("OXIDE_EMBED_BIN") {
        let p = p.trim().trim_matches('"').to_string();
        if !p.is_empty() {
            out.push(PathBuf::from(p));
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            out.push(dir.join("oxide-embed"));
            out.push(dir.join("binaries").join("oxide-embed"));
        }
    }
    out.push(PathBuf::from("/usr/lib/oxide-tech-local-agent/oxide-embed"));
    out.push(PathBuf::from("/usr/lib/oxide-agent/oxide-embed"));
    out.push(PathBuf::from("/usr/bin/oxide-embed"));
    out
}

/// Resolve the `oxide-embed` binary. Returns the path to execute and a flag
/// telling whether it was found on `PATH` (bare command) or as a file.
pub fn resolve_embed_bin() -> Result<PathBuf, String> {
    for cand in candidate_paths() {
        if cand.is_file() {
            return Ok(cand);
        }
    }
    // Fall back to PATH lookup.
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let cand = dir.join("oxide-embed");
            if cand.is_file() {
                return Ok(cand);
            }
        }
    }
    Err(
        "oxide-embed not found. Install it on PATH (~/.local/bin/oxide-embed), \
         set OXIDE_EMBED_BIN, or install the Oxide Agent .deb (ships the sidecar)."
            .to_string(),
    )
}

pub fn resolve_cwd(cwd: Option<String>) -> PathBuf {
    if let Some(c) = cwd {
        let c = c.trim();
        if !c.is_empty() {
            return PathBuf::from(c);
        }
    }
    if let Ok(p) = std::env::var("OXIDE_PROJECT_DIR") {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

async fn run_embed(args: &[String], cwd: &Path) -> Result<EmbedResult, String> {
    let bin = resolve_embed_bin()?;
    let output = tokio::process::Command::new(&bin)
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .map_err(|e| format!("Failed to execute '{}': {e}", bin.display()))?;
    Ok(EmbedResult {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
    })
}

/// Synchronous variant for CLI subcommands (doctor / memory passthrough).
pub fn run_embed_blocking(args: &[String], cwd: &Path) -> Result<std::process::Output, String> {
    let bin = resolve_embed_bin()?;
    std::process::Command::new(&bin)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("Failed to execute '{}': {e}", bin.display()))
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn memory_env(cwd: Option<String>) -> Result<MemoryEnv, String> {
    let bin = resolve_embed_bin()?;
    let dir = resolve_cwd(cwd);
    let version_out = tokio::process::Command::new(&bin)
        .arg("--version")
        .output()
        .await
        .map_err(|e| format!("Failed to execute '{}': {e}", bin.display()))?;
    let mut version = String::from_utf8_lossy(&version_out.stdout)
        .trim()
        .to_string();
    if version.is_empty() {
        version = String::from_utf8_lossy(&version_out.stderr)
            .trim()
            .to_string();
    }
    Ok(MemoryEnv {
        bin: bin.display().to_string(),
        version,
        cwd: dir.display().to_string(),
        manifest_present: dir.join(".oxide").join("manifest.json").is_file(),
    })
}

#[tauri::command]
pub async fn memory_status(cwd: Option<String>) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    run_embed(&["status".to_string()], &dir).await
}

#[tauri::command]
pub async fn memory_init(cwd: Option<String>, name: Option<String>) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["init".to_string()];
    if let Some(n) = name {
        if !n.trim().is_empty() {
            args.push("--name".to_string());
            args.push(n);
        }
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_index(cwd: Option<String>, force: Option<bool>) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["index".to_string()];
    if force.unwrap_or(false) {
        args.push("--force".to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_search(
    cwd: Option<String>,
    query: String,
    stair: Option<bool>,
    limit: Option<usize>,
    budget: Option<usize>,
    with_graph: Option<bool>,
) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["search".to_string(), query];
    if stair.unwrap_or(false) {
        args.push("--stair".to_string());
    }
    if let Some(l) = limit {
        args.push("--limit".to_string());
        args.push(l.to_string());
    }
    if let Some(b) = budget {
        args.push("--budget".to_string());
        args.push(b.to_string());
    }
    if with_graph.unwrap_or(false) {
        args.push("--with-graph".to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_context(
    cwd: Option<String>,
    task: String,
    budget: Option<usize>,
) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["context".to_string(), task];
    if let Some(b) = budget {
        args.push("--budget".to_string());
        args.push(b.to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_remember(
    cwd: Option<String>,
    content: String,
    kind: Option<String>,
    tags: Option<String>,
    symbol: Option<String>,
    auto_resolve: Option<bool>,
) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["remember".to_string(), content];
    if let Some(k) = kind {
        if !k.trim().is_empty() {
            args.push("--kind".to_string());
            args.push(k);
        }
    }
    if let Some(t) = tags {
        if !t.trim().is_empty() {
            args.push("--tags".to_string());
            args.push(t);
        }
    }
    if let Some(s) = symbol {
        if !s.trim().is_empty() {
            args.push("--symbol".to_string());
            args.push(s);
        }
    }
    if auto_resolve.unwrap_or(false) {
        args.push("--auto-resolve".to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_recall(
    cwd: Option<String>,
    query: String,
    kind: Option<String>,
    tags: Option<String>,
    budget: Option<usize>,
    limit: Option<usize>,
) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["recall".to_string(), query];
    if let Some(k) = kind {
        if !k.trim().is_empty() {
            args.push("--kind".to_string());
            args.push(k);
        }
    }
    if let Some(t) = tags {
        if !t.trim().is_empty() {
            args.push("--tags".to_string());
            args.push(t);
        }
    }
    if let Some(b) = budget {
        args.push("--budget".to_string());
        args.push(b.to_string());
    }
    if let Some(l) = limit {
        args.push("--limit".to_string());
        args.push(l.to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_explain(
    cwd: Option<String>,
    symbol: String,
    hops: Option<usize>,
) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    let mut args = vec!["explain".to_string(), symbol];
    if let Some(h) = hops {
        args.push("--hops".to_string());
        args.push(h.to_string());
    }
    run_embed(&args, &dir).await
}

#[tauri::command]
pub async fn memory_conflicts(cwd: Option<String>) -> Result<EmbedResult, String> {
    let dir = resolve_cwd(cwd);
    run_embed(&["conflicts".to_string()], &dir).await
}
