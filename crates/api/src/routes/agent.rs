use actix_web::{post, get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, warn};

use config_loader::AppConfig;
use gateway_router::GatewayRouter;
use rag_pipeline::RagPipeline;
use sandbox::execute_in_sandbox;
use surrealdb_service::client::SurrealClient;
use tree_sitter_service::parser::parse_file;

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub workspace_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallArgs {
    pub filepath: String,
    pub content: Option<String>,
    pub search_block: Option<String>,
    pub replace_block: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    /// `"write_file"` | `"apply_diff"`
    pub name: String,
    pub args: ToolCallArgs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorResponse {
    pub tool_calls: Vec<ToolCall>,
}

// ── Filesystem helpers ────────────────────────────────────────────────────────

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target" || name.starts_with('.') {
                continue;
            }
            collect_rs_files(&path, out);
        } else if path.extension().map_or(false, |e| e == "rs") {
            out.push(path);
        }
    }
}

fn strip_json_fences(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("```json").unwrap_or(s);
    let s = s.strip_prefix("```").unwrap_or(s);
    let s = s.strip_suffix("```").unwrap_or(s);
    s.trim().to_string()
}

// ── File tool helpers (always non-blocking) ───────────────────────────────────

async fn write_file_async(workspace: &str, filepath: &str, content: &str) -> Result<(), String> {
    let full = Path::new(workspace).join(filepath).to_owned();
    let content = content.to_owned();
    tokio::task::spawn_blocking(move || {
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("mkdir failed for {:?}: {}", full, e))?;
        }
        fs::write(&full, &content)
            .map_err(|e| format!("write failed for {:?}: {}", full, e))
    })
    .await
    .map_err(|e| format!("write_file task panicked: {e}"))??;
    Ok(())
}

async fn apply_diff_async(
    workspace: &str,
    filepath: &str,
    search: &str,
    replace: &str,
) -> Result<(), String> {
    let full = Path::new(workspace).join(filepath).to_owned();
    let search = search.to_owned();
    let replace = replace.to_owned();
    tokio::task::spawn_blocking(move || {
        let content = fs::read_to_string(&full)
            .map_err(|e| format!("read failed for {:?}: {}", full, e))?;
        if !content.contains(&search) {
            return Err(format!("apply_diff: search block not found in {:?}", full));
        }
        let new_content = content.replacen(&search, &replace, 1);
        fs::write(&full, new_content)
            .map_err(|e| format!("write failed for {:?}: {}", full, e))
    })
    .await
    .map_err(|e| format!("apply_diff task panicked: {e}"))??;
    Ok(())
}

// ── Coder system prompt ────────────────────────────────────────────────────────

const CODER_SYSTEM_PROMPT: &str = r#"You are the Execution Coder for Oxide-Tech Local Agent OS.
You receive a fully-structured architecture prompt from the Thinker model and
your job is to produce idiomatic, production-ready Rust code.

TOOLS AVAILABLE:
- `write_file(filepath, content)` — create a new file.
- `apply_diff(filepath, search_block, replace_block)` — patch an existing file.

RULES:
1. Follow the Thinker's plan exactly.  Do not add unrequested features.
2. All Rust code must compile cleanly under `cargo check`.
3. Use embedded-hal 1.0 trait paths (`embedded_hal::spi::SpiDevice`, etc.).
4. Use `#[no_std]` + `#[no_main]` for bare-metal targets unless told otherwise.
5. No `unwrap()` in library code — use `?` or explicit error handling.
6. Add `///` doc-comments to every public item.
7. Return ONLY valid JSON — no markdown fences, no explanation:

{
  "tool_calls": [
    { "name": "write_file", "args": { "filepath": "src/sensor.rs", "content": "..." } },
    { "name": "apply_diff", "args": { "filepath": "Cargo.toml",
        "search_block": "tokio = \"1\"",
        "replace_block": "tokio = \"1\"\nserde = \"1\"" } }
  ]
}"#;

const VERIFIER_SYSTEM_PROMPT: &str = r#"You are the Verification Agent for Oxide-Tech Local Agent OS.
The Coder has produced edits that failed `cargo check`.

Analyse the compiler stderr below and return a SPECIFIC correction mandate
telling the Coder exactly what to fix.  Be concise and precise.

Format:
VERIFICATION FAILED.
Target: <filename>
Error: <compiler message>
Analysis: <root cause>
Required Fix: <exact instruction>"#;

// ── Core generation logic ─────────────────────────────────────────────────────

pub async fn run_agent_generate(
    req: GenerateRequest,
    app_cfg: &AppConfig,
    rag_pipeline: Option<&RagPipeline>,
) -> Result<serde_json::Value, String> {
    let workspace = req
        .workspace_path
        .clone()
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    // ── 1. Build / retrieve workspace AST context ─────────────────────────────
    let db = SurrealClient::new()
        .await
        .map_err(|e| format!("SurrealDB connect failed: {e}"))?;

    let mut workspace_ctx = db
        .get_workspace_context()
        .await
        .unwrap_or_else(|_| String::new());

    if workspace_ctx.is_empty() || workspace_ctx.contains("empty") {
        let wp = workspace.clone();
        let symbols = tokio::task::spawn_blocking(move || {
            let mut files = Vec::new();
            collect_rs_files(Path::new(&wp), &mut files);
            files
                .iter()
                .filter_map(|f| {
                    let content = fs::read_to_string(f).ok()?;
                    parse_file(&content, &f.to_string_lossy()).ok()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .await
        .map_err(|e| format!("AST parse panicked: {e}"))?;

        let _ = db.save_symbols(&symbols).await;
        workspace_ctx = db
            .get_workspace_context()
            .await
            .unwrap_or_else(|_| "No context available.".to_string());
    }

    // ── 2. Build router from config ───────────────────────────────────────────
    let router = GatewayRouter::from_config(app_cfg);

    // ── 3. Thinker: reasoning + structured prompt generation ──────────────────
    let mut rag_context = String::new();
    if let Some(rp) = rag_pipeline {
        match rp.search(&req.prompt, app_cfg.rag.top_k).await {
            Ok(chunks) => {
                for (i, chunk) in chunks.iter().enumerate() {
                    rag_context.push_str(&format!(
                        "--- Chunk {} (Source: {}, Crate: {:?}, File: {:?}) ---\n{}\n\n",
                        i + 1,
                        chunk.source,
                        chunk.crate_name,
                        chunk.file_path,
                        chunk.text
                    ));
                }
            }
            Err(e) => warn!("RAG search failed: {}", e),
        }
    }

    // Style context — Phase 5 will populate it.
    let style_context = String::new();

    let thinker_output = router
        .thinker
        .analyze_and_prompt(&workspace_ctx, &req.prompt, &rag_context, &style_context)
        .await
        .map_err(|e| format!("Thinker failed: {e}"))?;

    info!(
        pattern = %thinker_output.chosen_pattern,
        "Thinker completed — routing to coder"
    );

    // ── 4. Network probe → select coder backend ───────────────────────────────
    let primary_url = &app_cfg.coder.online.primary.base_url;
    let secondary_url = &app_cfg.coder.online.secondary.base_url;
    let mut backend = router.select_backend(primary_url, secondary_url).await;

    info!(backend = %backend, "GatewayRouter selected backend");

    // ── 5. Coder → Verifier auto-healing loop (max_retries) ──────────────────
    let max_retries = app_cfg.gateway.max_retries;
    let mut retry = 0usize;
    let mut current_coder_prompt = thinker_output.coder_prompt.clone();

    loop {
        let coder = router.client_for(backend);
        let raw = coder
            .complete(CODER_SYSTEM_PROMPT, &current_coder_prompt, true)
            .await
            .map_err(|e| {
                coder.warn_backend_failure(&e.to_string());
                format!("Coder ({backend}) failed: {e}")
            })?;

        let cleaned = strip_json_fences(&raw);
        let editor: EditorResponse = serde_json::from_str(&cleaned).map_err(|e| {
            format!("Coder returned invalid JSON: {e}\nRaw output:\n{raw}")
        })?;

        // Apply all tool calls.
        for tool in &editor.tool_calls {
            match tool.name.as_str() {
                "write_file" => {
                    if let Some(ref content) = tool.args.content {
                        write_file_async(&workspace, &tool.args.filepath, content).await?;
                    }
                }
                "apply_diff" => {
                    if let (Some(ref search), Some(ref replace)) =
                        (&tool.args.search_block, &tool.args.replace_block)
                    {
                        apply_diff_async(&workspace, &tool.args.filepath, search, replace)
                            .await?;
                    }
                }
                other => warn!("Unknown tool call '{}' — skipping", other),
            }
        }

        // cargo check via native sandbox.
        let check = execute_in_sandbox(&["cargo", "check"], &workspace)
            .await
            .map_err(|e| format!("Sandbox execution failed: {e}"))?;

        if check.exit_code == 0 {
            info!(backend = %backend, attempt = retry + 1, "cargo check PASSED");
            return Ok(serde_json::json!({
                "status": "PASSED",
                "backend_used": backend.to_string(),
                "pattern": thinker_output.chosen_pattern,
                "architecture_notes": thinker_output.architecture_notes,
                "attempts": retry + 1,
            }));
        }

        // Quality gate: should we switch backend before retrying?
        if let Some(better_backend) = router.evaluate_quality(&check.stderr, backend) {
            warn!(
                old = %backend,
                new = %better_backend,
                "Quality gate triggered backend flip"
            );
            backend = better_backend;
        }

        retry += 1;
        if retry >= max_retries {
            return Ok(serde_json::json!({
                "status": "FAILED",
                "backend_used": backend.to_string(),
                "attempts": retry,
                "stderr": check.stderr,
                "stdout": check.stdout,
            }));
        }

        // Verifier: produce a targeted correction mandate.
        let verifier_user = format!(
            "Exit code: {}\n\nStderr:\n{}\n\nPrevious coder output:\n{}",
            check.exit_code, check.stderr, cleaned
        );

        let correction = router
            .local // Verifier always uses local to avoid rate limits.
            .complete(VERIFIER_SYSTEM_PROMPT, &verifier_user, false)
            .await
            .map_err(|e| format!("Verifier failed: {e}"))?;

        current_coder_prompt = format!(
            "{}\n\n---CORRECTION MANDATE---\n{}\n\nApply the fix and return updated JSON tool calls.",
            current_coder_prompt, correction
        );
    }
}

// ── Actix-web handler ─────────────────────────────────────────────────────────

#[post("/api/agent/generate")]
pub async fn handle_agent_generate(
    req: web::Json<GenerateRequest>,
    cfg: web::Data<AppConfig>,
    rag: web::Data<Option<Arc<RagPipeline>>>,
) -> impl Responder {
    let rag_ref = rag.get_ref().as_ref().map(|p| p.as_ref());
    match run_agent_generate(req.into_inner(), cfg.get_ref(), rag_ref).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e,
        })),
    }
}

#[post("/api/rag/update")]
pub async fn handle_rag_update(
    cfg: web::Data<AppConfig>,
    rag: web::Data<Option<Arc<RagPipeline>>>,
) -> impl Responder {
    if let Some(ref rp) = *rag.get_ref() {
        let rp_clone = rp.clone();
        let watchlist = cfg.rag.watchlist.clone();

        tokio::spawn(async move {
            if let Err(e) = rag_pipeline::updater::check_and_update_crates(&watchlist, &rp_clone).await {
                tracing::error!("Background RAG update failed: {}", e);
            }
        });

        HttpResponse::Ok().json(serde_json::json!({
            "status": "triggered",
            "message": "Crate RAG update started in the background."
        }))
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "error",
            "message": "RAG pipeline is not initialized (check Qdrant connection)."
        }))
    }
}

#[get("/api/status")]
pub async fn handle_status(cfg: web::Data<AppConfig>) -> impl Responder {
    let router = GatewayRouter::from_config(cfg.get_ref());
    HttpResponse::Ok().json(router.status_summary())
}
