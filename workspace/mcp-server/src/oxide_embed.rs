use schemars::JsonSchema;
use serde::Deserialize;
use std::path::Path;
use tokio::process::Command;

#[derive(Deserialize, JsonSchema)]
pub struct OxideStairSearchInput {
    /// Symbol, function, struct or code structure query
    pub query: String,
    /// Maximum number of hierarchical hits
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideContextInput {
    /// The engineering or coding task description
    pub task: String,
    /// Maximum token budget ceiling (default 1500)
    pub budget: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideSearchInput {
    /// Search text, symbol, or identifier
    pub query: String,
    /// Optional token budget
    pub budget: Option<usize>,
    /// Include AST call and doc graph
    pub with_graph: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideRememberInput {
    /// Semantic memory statement or assertion to store
    pub content: String,
    /// Category: instruction, decision, preference, fact, goal, learning, etc.
    pub kind: Option<String>,
    /// Optional tags
    pub tags: Option<Vec<String>>,
    /// Target symbol governed by this memory
    pub symbol: Option<String>,
    /// Automatically supersede conflicting older memory
    pub auto_resolve: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideRecallInput {
    /// Topic or query to recall
    pub query: String,
    /// Category filter
    pub kind: Option<String>,
    /// Tag filters
    pub tags: Option<Vec<String>>,
    /// Token budget ceiling
    pub budget: Option<usize>,
    /// Maximum results (default 5)
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideExplainInput {
    /// Symbol name to explain
    pub symbol: String,
    /// Graph traversal depth (default 2)
    pub hops: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct OxideReadSymbolInput {
    /// Relative path to file
    pub file_path: String,
    /// Symbol name to slice
    pub symbol: String,
}

pub async fn run_oxide_embed_cmd(args: &[&str], cwd: &Path) -> Result<String, String> {
    let output = Command::new("oxide-embed")
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .map_err(|e| format!("Failed to execute 'oxide-embed': {e}. Ensure oxide-embed is installed and on PATH."))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        if stdout.is_empty() && !stderr.is_empty() {
            Ok(stderr)
        } else {
            Ok(stdout)
        }
    } else {
        Err(format!(
            "oxide-embed failed (code {}): {}",
            output.status.code().unwrap_or(-1),
            if !stderr.is_empty() { stderr } else { stdout }
        ))
    }
}

pub async fn stair_search(query: &str, limit: Option<usize>, cwd: &Path) -> Result<String, String> {
    let mut args = vec!["search", query, "--stair"];
    let limit_str;
    if let Some(lim) = limit {
        limit_str = lim.to_string();
        args.push("--limit");
        args.push(&limit_str);
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn context(task: &str, budget: Option<usize>, cwd: &Path) -> Result<String, String> {
    let mut args = vec!["context", task];
    let budget_str;
    if let Some(b) = budget {
        budget_str = b.to_string();
        args.push("--budget");
        args.push(&budget_str);
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn search(
    query: &str,
    budget: Option<usize>,
    with_graph: Option<bool>,
    cwd: &Path,
) -> Result<String, String> {
    let mut args = vec!["search", query];
    let budget_str;
    if let Some(b) = budget {
        budget_str = b.to_string();
        args.push("--budget");
        args.push(&budget_str);
    }
    if with_graph.unwrap_or(false) {
        args.push("--with-graph");
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn remember(
    content: &str,
    kind: Option<&str>,
    tags: Option<&[String]>,
    symbol: Option<&str>,
    auto_resolve: Option<bool>,
    cwd: &Path,
) -> Result<String, String> {
    let mut args = vec!["remember", content];
    if let Some(k) = kind {
        args.push("--kind");
        args.push(k);
    }
    let tags_joined;
    if let Some(t) = tags && !t.is_empty() {
        tags_joined = t.join(",");
        args.push("--tags");
        args.push(&tags_joined);
    }
    if let Some(s) = symbol {
        args.push("--symbol");
        args.push(s);
    }
    if auto_resolve.unwrap_or(false) {
        args.push("--auto-resolve");
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn recall(
    query: &str,
    kind: Option<&str>,
    tags: Option<&[String]>,
    budget: Option<usize>,
    limit: Option<usize>,
    cwd: &Path,
) -> Result<String, String> {
    let mut args = vec!["recall", query];
    if let Some(k) = kind {
        args.push("--kind");
        args.push(k);
    }
    let tags_joined;
    if let Some(t) = tags && !t.is_empty() {
        tags_joined = t.join(",");
        args.push("--tags");
        args.push(&tags_joined);
    }
    let budget_str;
    if let Some(b) = budget {
        budget_str = b.to_string();
        args.push("--budget");
        args.push(&budget_str);
    }
    let limit_str;
    if let Some(lim) = limit {
        limit_str = lim.to_string();
        args.push("--limit");
        args.push(&limit_str);
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn explain(symbol: &str, hops: Option<usize>, cwd: &Path) -> Result<String, String> {
    let mut args = vec!["explain", symbol];
    let hops_str;
    if let Some(h) = hops {
        hops_str = h.to_string();
        args.push("--hops");
        args.push(&hops_str);
    }
    run_oxide_embed_cmd(&args, cwd).await
}

pub async fn read_symbol(file_path: &str, symbol: &str, cwd: &Path) -> Result<String, String> {
    let args = vec!["read", file_path, "--symbol", symbol];
    run_oxide_embed_cmd(&args, cwd).await
}
