use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::client::LlmRouterClient;
use config_loader::ModelConfig;

// ── Thinker output ────────────────────────────────────────────────────────────

/// The structured output produced by the Thinker model.
///
/// The Thinker never writes code.  It reasons about architecture, selects
/// design patterns, identifies which files to touch, and then distils all
/// of that reasoning into a single `coder_prompt` that the downstream coder
/// model can act on.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThinkerOutput {
    /// The design pattern or architectural approach chosen.
    /// e.g. "Embassy async executor with interrupt-driven DMA"
    pub chosen_pattern: String,
    /// Why this pattern was selected given the workspace context and HAL version.
    pub architecture_notes: String,
    /// Files the coder must create or modify (relative to workspace root).
    pub files_to_touch: Vec<String>,
    /// The fully-formed, context-rich prompt that will be sent verbatim to the
    /// coder model.  Must be self-contained (no implicit context assumed).
    pub coder_prompt: String,
}

// ── Embedded-Rust–aware system prompt ────────────────────────────────────────

const THINKER_SYSTEM_PROMPT: &str = r#"
You are the Chief Architecture Thinker for an Embedded Rust development agent
called Oxide-Tech. Your sole job is to REASON and PLAN — you never write
production code yourself.

═══════════════════════════════════════════════════════════════════════════════
DOMAIN KNOWLEDGE (always apply these rules)
═══════════════════════════════════════════════════════════════════════════════
• Embedded-hal 1.0 (stable, released 2024): use the new trait path
    `embedded_hal::spi::SpiDevice`, `embedded_hal::i2c::I2c`,
    `embedded_hal::digital::OutputPin`, etc.
    NEVER use the 0.2.x `embedded_hal::blocking::*` path.
• Embassy async runtime: prefer `embassy_executor::task` + `embassy_time`
    for async embedded code on cortex-m, rp2040, nrf52, esp32.
    Use `embassy_sync::mutex::Mutex` with `ThreadModeRawMutex` for shared
    peripheral access.
• RTIC 2.x: use the monotonic + software task model.  RTIC is preferred over
    Embassy when deterministic interrupt priorities matter (safety-critical).
• `#[no_std]` + `#[no_main]` is mandatory for bare-metal targets.
• defmt + probe-rs is the logging and flashing stack (not RTT manually).
• heapless collections (Vec, String, FnvIndexMap) replace std collections.
• `nb::Result` is used for non-blocking HAL operations that the old API relied
    on; the new embedded-hal 1.0 traits are blocking by default.
• cortex-m-rt `#[entry]` is the bare-metal entry point for non-Embassy targets.

═══════════════════════════════════════════════════════════════════════════════
CODING STYLE RULES (match the user's existing codebase)
═══════════════════════════════════════════════════════════════════════════════
• Errors: use `thiserror` for library crates, `anyhow` for binary/app crates.
• Async: `tokio` for host-side code; `embassy` for target-side code.
• No `unwrap()` in library code — use `?` or explicit `match`.
• Naming: snake_case for functions/variables, CamelCase for types/traits.
• Doc-comments on every public item (`///`).
• Derive `Debug`, `Clone`, and `serde::{Serialize, Deserialize}` on all data
    structs that cross async boundaries.

═══════════════════════════════════════════════════════════════════════════════
YOUR OUTPUT CONTRACT
═══════════════════════════════════════════════════════════════════════════════
Return ONLY a single JSON object matching this schema — no markdown fences,
no explanation outside the JSON:

{
  "chosen_pattern": "concise pattern name",
  "architecture_notes": "why this pattern fits; HAL version choices; any tradeoffs",
  "files_to_touch": ["relative/path/to/file.rs", ...],
  "coder_prompt": "A complete, self-contained instruction for the Coder model.
                   Include: exact trait paths, struct names, function signatures,
                   which files to create vs. modify, Cargo.toml changes needed.
                   Reference the workspace context facts provided to you."
}
"#;

// ── ThinkerClient ─────────────────────────────────────────────────────────────

/// Wraps `LlmRouterClient` with the embedded-Rust reasoning system prompt.
pub struct ThinkerClient {
    inner: LlmRouterClient,
}

impl ThinkerClient {
    /// Build from a `ModelConfig` (usually `config.thinker`).
    pub fn from_config(cfg: &ModelConfig) -> Self {
        Self {
            inner: LlmRouterClient::from_config(cfg),
        }
    }

    /// Analyse the workspace context + user mandate + RAG chunks and produce a
    /// `ThinkerOutput` with a ready-to-use coder prompt.
    ///
    /// `workspace_context` — the textual AST summary from SurrealDB.
    /// `user_mandate`      — the raw user request.
    /// `rag_context`       — top-k RAG chunks from the Qdrant pipeline.
    /// `style_context`     — the user's coding style profile summary.
    pub async fn analyze_and_prompt(
        &self,
        workspace_context: &str,
        user_mandate: &str,
        rag_context: &str,
        style_context: &str,
    ) -> Result<ThinkerOutput, anyhow::Error> {
        let user_msg = format!(
            "═══ WORKSPACE AST CONTEXT ═══\n{workspace_context}\n\n\
             ═══ RELEVANT CRATE DOCUMENTATION (RAG) ═══\n{rag_context}\n\n\
             ═══ USER CODING STYLE PROFILE ═══\n{style_context}\n\n\
             ═══ USER MANDATE ═══\n{user_mandate}\n\n\
             Analyse the above information and return your JSON ThinkerOutput."
        );

        info!(mandate = %user_mandate, "Thinker: starting analysis");

        let raw = self
            .inner
            .complete(THINKER_SYSTEM_PROMPT, &user_msg, true)
            .await
            .context("Thinker LLM call failed")?;

        let cleaned = strip_json_fences(&raw);

        let output: ThinkerOutput = serde_json::from_str(&cleaned)
            .with_context(|| format!("Thinker returned invalid JSON:\n{raw}"))?;

        info!(
            pattern = %output.chosen_pattern,
            files = ?output.files_to_touch,
            "Thinker: analysis complete"
        );

        Ok(output)
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn strip_json_fences(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("```json").unwrap_or(s);
    let s = s.strip_prefix("```").unwrap_or(s);
    let s = s.strip_suffix("```").unwrap_or(s);
    s.trim().to_string()
}
