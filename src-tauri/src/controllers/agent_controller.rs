//! Agent Controller
//!
//! Consolidates inference execution, dynamic VRAM model admission,
//! prompt pipelines, skills studio, model arena, and hub downloader.

use serde::{Deserialize, Serialize};

// Re-export all underlying IPC commands and their Tauri macro wrappers
pub use crate::arena_ipc::*;
pub use crate::hub_ipc::*;
pub use crate::model_ipc::*;
pub use crate::skills_ipc::*;

/// Strictly typed request DTO for in-process agent prompt execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub model: String,
    pub provider: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// Strictly typed response DTO for agent completions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub text: String,
    pub model: String,
    pub tokens_used: Option<usize>,
    pub latency_ms: u64,
}

/// Execute a prompt directly against the local agent runtime with strict typed boundary.
#[tauri::command]
pub async fn execute_agent_prompt(req: PromptRequest) -> Result<PromptResponse, String> {
    let resp = crate::model_ipc::model_run_prompt(crate::model_ipc::RunPromptRequest {
        prompt: req.prompt,
        system_prompt: None,
        model: req.model,
        provider: req.provider,
        base_url: None,
        temperature: req.temperature,
        max_tokens: req.max_tokens,
        stair_context: None,
    })
    .await?;

    if let Some(err) = resp.error {
        return Err(err);
    }

    Ok(PromptResponse {
        text: resp.text,
        model: resp.model,
        tokens_used: resp.tokens_used,
        latency_ms: resp.latency_ms,
    })
}
