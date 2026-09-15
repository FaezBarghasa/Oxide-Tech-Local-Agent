use crate::provider::{
    BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
    ProviderKind, StreamChunk, StreamResult,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;
use tokio_stream::StreamExt;

pub struct OllamaProvider {
    base_url: String,
    default_model: String,
    client: Client,
}

impl OllamaProvider {
    pub fn new(base_url: Option<String>, default_model: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            default_model: default_model.unwrap_or_else(|| "qwen2.5-coder:7b".to_string()),
            client: Client::builder().build().unwrap_or_default(),
        }
    }
}

#[async_trait]
impl InferenceProvider for OllamaProvider {
    fn capabilities(&self) -> InferenceCapabilities {
        InferenceCapabilities {
            provider: ProviderKind::Ollama,
            supports_streaming: true,
            supports_tool_calls: true,
            supports_lora_hotswap: false,
            supports_json_mode: true,
            supports_grammar_constrained: false,
            supports_speculative_decoding: false,
            supports_prefix_cache: false,
            supports_multimodal: false,
            context_window: 32768,
        }
    }

    async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> {
        let start = Instant::now();
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
            "stream": false,
            "options": {
                "temperature": req.temperature.unwrap_or(0.2),
            }
        });

        if let Some(true) = req.json_mode {
            body["format"] = json!("json");
        }

        let url = format!("{}/api/chat", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to connect to Ollama backend")?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Ollama error (HTTP {}): {}", status, err_text));
        }

        let res_json: serde_json::Value = resp.json().await?;
        let content = res_json["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let prompt_tokens = res_json["prompt_eval_count"].as_u64().unwrap_or(0) as usize;
        let completion_tokens = res_json["eval_count"].as_u64().unwrap_or(0) as usize;
        let latency_ms = start.elapsed().as_millis() as u64;

        Ok(ChatResponse {
            content,
            prompt_tokens,
            completion_tokens,
            finish_reason: Some("stop".to_string()),
            tool_calls: Vec::new(),
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

        let body = json!({
            "model": model,
            "messages": messages_json,
            "stream": true,
            "options": {
                "temperature": req.temperature.unwrap_or(0.2),
            }
        });

        let url = format!("{}/api/chat", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to start Ollama stream")?;

        let byte_stream = resp.bytes_stream();
        let chunk_stream = byte_stream.map(|item| -> Result<StreamChunk> {
            let bytes = item.context("Stream read error")?;
            if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                let delta = val["message"]["content"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let is_final = val["done"].as_bool().unwrap_or(false);
                Ok(StreamChunk {
                    delta,
                    is_final,
                    tool_calls_delta: vec![],
                })
            } else {
                Ok(StreamChunk {
                    delta: String::new(),
                    is_final: false,
                    tool_calls_delta: vec![],
                })
            }
        });

        Ok(Box::pin(chunk_stream))
    }

    async fn health(&self) -> Result<BackendHealth> {
        let url = format!("{}/api/version", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => {
                let healthy = resp.status().is_success();
                Ok(BackendHealth {
                    healthy,
                    provider_name: "Ollama".to_string(),
                    active_model: self.default_model.clone(),
                    memory_used_mb: None,
                    vram_used_mb: None,
                    available_slots: None,
                    queue_depth: None,
                })
            }
            Err(_) => Ok(BackendHealth {
                healthy: false,
                provider_name: "Ollama (unreachable)".to_string(),
                active_model: self.default_model.clone(),
                memory_used_mb: None,
                vram_used_mb: None,
                available_slots: None,
                queue_depth: None,
            }),
        }
    }
}
