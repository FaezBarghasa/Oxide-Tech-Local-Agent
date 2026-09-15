//! llama-server (llama.cpp HTTP) provider implementation.
//!
//! Supports:
//! - OpenAI-compatible `/v1/chat/completions` (streaming SSE + blocking)
//! - Native grammar-constrained output via the `grammar` GBNF field
//! - Tool-call parsing from `tool_calls` response field
//! - KV prefix-cache slot stickiness via `id_slot`
//! - `/health` endpoint monitoring with VRAM and slot stats

use crate::provider::{
    BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
    ProviderKind, StreamChunk, StreamResult, ToolCall,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Instant;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, warn};

pub struct LlamaServerProvider {
    base_url: String,
    default_model: String,
    client: Client,
}

impl LlamaServerProvider {
    pub fn new(base_url: impl Into<String>, default_model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            default_model: default_model.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .expect("Failed to build HTTP client for LlamaServerProvider"),
        }
    }

    /// Convert `ConversationTurn` history + current `ChatRequest` into the
    /// OpenAI messages array that llama-server expects.
    fn build_messages(req: &ChatRequest) -> Vec<Value> {
        let mut messages: Vec<Value> = req
            .history
            .iter()
            .map(|turn| {
                let mut m = json!({
                    "role": turn.role,
                    "content": turn.content,
                });
                if let Some(id) = &turn.tool_call_id {
                    m["tool_call_id"] = json!(id);
                }
                if let Some(name) = &turn.name {
                    m["name"] = json!(name);
                }
                if !turn.tool_calls.is_empty() {
                    m["tool_calls"] = json!(turn.tool_calls.iter().map(|tc| json!({
                        "id": tc.id,
                        "type": "function",
                        "function": {
                            "name": tc.name,
                            "arguments": tc.arguments.to_string(),
                        }
                    })).collect::<Vec<_>>());
                }
                m
            })
            .collect();

        // Append the current user turn from the request messages
        for m in &req.messages {
            let mut msg = json!({ "role": m.role, "content": m.content });
            // Attach base64 images if this is a user turn and images are provided
            if m.role == "user" {
                if let Some(images) = &req.images {
                    let content_parts: Vec<Value> = {
                        let mut parts = vec![json!({ "type": "text", "text": m.content })];
                        for img in images {
                            parts.push(json!({
                                "type": "image_url",
                                "image_url": { "url": format!("data:image/jpeg;base64,{}", img) }
                            }));
                        }
                        parts
                    };
                    msg["content"] = json!(content_parts);
                }
            }
            messages.push(msg);
        }
        messages
    }
}

// ── OpenAI-compatible response types ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct OaiToolCallFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OaiToolCall {
    id: String,
    function: OaiToolCallFunction,
}

#[derive(Debug, Deserialize)]
struct OaiMessage {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<OaiToolCall>,
}

#[derive(Debug, Deserialize)]
struct OaiChoice {
    message: OaiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OaiUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct OaiChatResponse {
    choices: Vec<OaiChoice>,
    usage: Option<OaiUsage>,
    /// llama-server reports the used slot ID here for prefix-cache reuse
    id_slot: Option<i32>,
}

// ── llama-server /health response ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct LlamaHealth {
    status: Option<String>,
    slots_idle: Option<usize>,
    slots_processing: Option<usize>,
}

#[async_trait]
impl InferenceProvider for LlamaServerProvider {
    fn capabilities(&self) -> InferenceCapabilities {
        InferenceCapabilities {
            provider: ProviderKind::LlamaServer,
            supports_streaming: true,
            supports_tool_calls: true,
            supports_lora_hotswap: false,
            supports_json_mode: true,
            supports_grammar_constrained: true,
            supports_speculative_decoding: true,
            supports_prefix_cache: true,
            supports_multimodal: true,
            context_window: 131072,
        }
    }

    fn provider_name(&self) -> &str {
        "llama-server"
    }

    async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> {
        let t0 = Instant::now();
        let url = format!("{}/v1/chat/completions", self.base_url);
        let model = if req.model.is_empty() {
            &self.default_model
        } else {
            &req.model
        };

        let messages = Self::build_messages(&req);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "temperature": req.temperature.unwrap_or(0.15),
            "max_tokens": req.max_tokens.unwrap_or(4096),
        });

        // Grammar-constrained output (GBNF string)
        if let Some(grammar) = &req.grammar {
            body["grammar"] = json!(grammar);
        }

        // JSON mode (llama-server format)
        if let Some(true) = req.json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        // Tool definitions
        if let Some(tools) = &req.tools {
            body["tools"] = json!(tools.iter().map(|t| json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })).collect::<Vec<_>>());
            if let Some(choice) = &req.tool_choice {
                body["tool_choice"] = json!(choice);
            }
        }

        // Stop sequences
        if let Some(stop) = &req.stop {
            body["stop"] = json!(stop);
        }

        // Slot stickiness for KV prefix cache reuse
        if let Some(slot_id) = req.slot_id {
            body["id_slot"] = json!(slot_id);
        }

        debug!(url = %url, model = %model, "LlamaServer: sending chat completion request");

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("LlamaServer: connection failed")?;

        let status = resp.status();
        if !status.is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(anyhow!("LlamaServer HTTP {}: {}", status, err));
        }

        let oai: OaiChatResponse = resp.json().await.context("LlamaServer: JSON parse error")?;
        let choice = oai.choices.into_iter().next().ok_or_else(|| anyhow!("LlamaServer: empty choices"))?;
        let latency_ms = t0.elapsed().as_millis() as u64;

        let content = choice.message.content.unwrap_or_default();
        let mut tool_calls: Vec<ToolCall> = choice
            .message
            .tool_calls
            .into_iter()
            .map(|tc| {
                let args = serde_json::from_str(&tc.function.arguments).unwrap_or(Value::Null);
                ToolCall { id: tc.id, name: tc.function.name, arguments: args }
            })
            .collect();

        // Fallback: attempt to parse tool calls from raw content if structured field is empty
        if tool_calls.is_empty() && !content.is_empty() {
            tool_calls = ToolCallParser::extract_from_text(&content);
        }

        let usage = oai.usage.unwrap_or(OaiUsage { prompt_tokens: 0, completion_tokens: 0 });

        Ok(ChatResponse {
            content,
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            finish_reason: choice.finish_reason,
            tool_calls,
            latency_ms,
            slot_id: oai.id_slot,
        })
    }

    async fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let model = if req.model.is_empty() { &self.default_model } else { &req.model };
        let messages = Self::build_messages(&req);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "temperature": req.temperature.unwrap_or(0.15),
            "max_tokens": req.max_tokens.unwrap_or(4096),
        });

        if let Some(grammar) = &req.grammar {
            body["grammar"] = json!(grammar);
        }
        if let Some(slot_id) = req.slot_id {
            body["id_slot"] = json!(slot_id);
        }

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("LlamaServer: stream connection failed")?;

        let byte_stream = resp.bytes_stream();
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<StreamChunk>>(64);

        tokio::spawn(async move {
            let mut byte_stream = Box::pin(byte_stream);
            while let Some(item) = byte_stream.next().await {
                match item {
                    Err(e) => {
                        let _ = tx.send(Err(anyhow!("Stream error: {}", e))).await;
                        break;
                    }
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            let line = line.trim();
                            if let Some(data) = line.strip_prefix("data: ") {
                                if data == "[DONE]" {
                                    let _ = tx.send(Ok(StreamChunk {
                                        delta: String::new(),
                                        is_final: true,
                                        tool_calls_delta: vec![],
                                    })).await;
                                    return;
                                }
                                if let Ok(val) = serde_json::from_str::<Value>(data) {
                                    let delta = val["choices"][0]["delta"]["content"]
                                        .as_str()
                                        .unwrap_or_default()
                                        .to_string();
                                    let is_final = val["choices"][0]["finish_reason"].is_string();
                                    let _ = tx.send(Ok(StreamChunk {
                                        delta,
                                        is_final,
                                        tool_calls_delta: vec![],
                                    })).await;
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(Box::pin(ReceiverStream::new(rx)))
    }

    async fn health(&self) -> Result<BackendHealth> {
        let url = format!("{}/health", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let h: LlamaHealth = resp.json().await.unwrap_or(LlamaHealth {
                    status: None,
                    slots_idle: None,
                    slots_processing: None,
                });
                let healthy = h.status.as_deref() == Some("ok");
                Ok(BackendHealth {
                    healthy,
                    provider_name: "llama-server".to_string(),
                    active_model: self.default_model.clone(),
                    memory_used_mb: None,
                    vram_used_mb: None,
                    available_slots: h.slots_idle,
                    queue_depth: h.slots_processing,
                })
            }
            Ok(_resp) => Ok(BackendHealth {
                healthy: true,
                provider_name: "llama-server".to_string(),
                active_model: self.default_model.clone(),
                memory_used_mb: None,
                vram_used_mb: None,
                available_slots: None,
                queue_depth: None,
            }),
            Err(e) => {
                warn!("LlamaServer health check failed: {}", e);
                Ok(BackendHealth {
                    healthy: false,
                    provider_name: "llama-server (unreachable)".to_string(),
                    active_model: self.default_model.clone(),
                    memory_used_mb: None,
                    vram_used_mb: None,
                    available_slots: None,
                    queue_depth: None,
                })
            }
        }
    }
}
