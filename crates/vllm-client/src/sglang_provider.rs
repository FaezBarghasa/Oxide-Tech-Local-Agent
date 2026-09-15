use crate::provider::{
    BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
    ProviderKind, StreamChunk, StreamResult, ToolCall,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;
use tokio_stream::StreamExt;

/// High-throughput SGLang inference provider using RadixAttention prefix caching and structured endpoints.
pub struct SglangProvider {
    base_url: String,
    default_model: String,
    client: Client,
    draft_model: Option<String>,
}

impl SglangProvider {
    pub fn new(
        base_url: Option<String>,
        default_model: Option<String>,
        draft_model: Option<String>,
    ) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:30000".to_string()),
            default_model: default_model.unwrap_or_else(|| "default".to_string()),
            client: Client::builder().build().unwrap_or_default(),
            draft_model,
        }
    }
}

#[async_trait]
impl InferenceProvider for SglangProvider {
    fn capabilities(&self) -> InferenceCapabilities {
        InferenceCapabilities {
            provider: ProviderKind::Sglang,
            supports_streaming: true,
            supports_tool_calls: true,
            supports_lora_hotswap: false,
            supports_json_mode: true,
            supports_grammar_constrained: true,
            supports_speculative_decoding: self.draft_model.is_some(),
            supports_prefix_cache: true,
            supports_multimodal: true,
            context_window: 131072,
        }
    }

    async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> {
        let start = Instant::now();
        let model = if req.model.is_empty() {
            &self.default_model
        } else {
            &req.model
        };

        let mut messages_json = Vec::new();
        for m in req.messages {
            messages_json.push(json!({
                "role": m.role,
                "content": m.content,
            }));
        }

        let mut body = json!({
            "model": model,
            "messages": messages_json,
            "stream": false,
            "temperature": req.temperature.unwrap_or(0.2),
            "max_tokens": req.max_tokens.unwrap_or(4096),
        });

        if let Some(tools) = req.tools {
            if !tools.is_empty() {
                body["tools"] = json!(tools);
            }
        }

        if let Some(grammar) = req.grammar {
            body["ebnf"] = json!(grammar);
        }

        if let Some(true) = req.json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        let url = format!("{}/v1/chat/completions", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to connect to SGLang backend")?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("SGLang HTTP error {}: {}", status, err_text));
        }

        let res_json: serde_json::Value = resp.json().await?;
        let choice = &res_json["choices"][0];
        let content = choice["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let tool_calls: Vec<ToolCall> = choice["message"]["tool_calls"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let id = t["id"].as_str().unwrap_or("").to_string();
                        let name = t["function"]["name"].as_str()?.to_string();
                        let args = t["function"]["arguments"].as_str().unwrap_or("{}");
                        let parsed_args = serde_json::from_str(args).unwrap_or(json!({}));
                        Some(ToolCall {
                            id,
                            name,
                            arguments: parsed_args,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let prompt_tokens = res_json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as usize;
        let completion_tokens =
            res_json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as usize;
        let finish_reason = choice["finish_reason"].as_str().map(String::from);
        let latency_ms = start.elapsed().as_millis() as u64;

        Ok(ChatResponse {
            content,
            prompt_tokens,
            completion_tokens,
            finish_reason,
            tool_calls,
            latency_ms,
            slot_id: None,
        })
    }

    async fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult> {
        let model = if req.model.is_empty() {
            &self.default_model
        } else {
            &req.model
        };

        let messages_json: Vec<serde_json::Value> = req
            .messages
            .into_iter()
            .map(|m| json!({ "role": m.role, "content": m.content }))
            .collect();

        let mut body = json!({
            "model": model,
            "messages": messages_json,
            "stream": true,
            "temperature": req.temperature.unwrap_or(0.2),
            "max_tokens": req.max_tokens.unwrap_or(4096),
        });

        if let Some(grammar) = req.grammar {
            body["ebnf"] = json!(grammar);
        }

        let url = format!("{}/v1/chat/completions", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to start SGLang stream")?;

        let byte_stream = resp.bytes_stream();
        let chunk_stream = byte_stream.map(|item| -> Result<StreamChunk> {
            let bytes = item.context("Stream read error")?;
            let text = String::from_utf8_lossy(&bytes);

            for line in text.lines() {
                let line = line.trim();
                if line.starts_with("data: ") {
                    let data = &line["data: ".len()..];
                    if data == "[DONE]" {
                        return Ok(StreamChunk {
                            delta: String::new(),
                            is_final: true,
                            tool_calls_delta: vec![],
                        });
                    }
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                        let delta = val["choices"][0]["delta"]["content"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        return Ok(StreamChunk {
                            delta,
                            is_final: false,
                            tool_calls_delta: vec![],
                        });
                    }
                }
            }

            Ok(StreamChunk {
                delta: String::new(),
                is_final: false,
                tool_calls_delta: vec![],
            })
        });

        Ok(Box::pin(chunk_stream))
    }

    async fn health(&self) -> Result<BackendHealth> {
        let url = format!("{}/health", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => Ok(BackendHealth {
                healthy: true,
                provider_name: "SGLang (RadixAttention)".to_string(),
                active_model: self.default_model.clone(),
                memory_used_mb: None,
                vram_used_mb: None,
                available_slots: None,
                queue_depth: None,
            }),
            _ => Ok(BackendHealth {
                healthy: false,
                provider_name: "SGLang (unreachable)".to_string(),
                active_model: self.default_model.clone(),
                memory_used_mb: None,
                vram_used_mb: None,
                available_slots: None,
                queue_depth: None,
            }),
        }
    }
}
