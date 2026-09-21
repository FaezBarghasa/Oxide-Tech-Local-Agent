use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
    traits::{self, ChatMessage, EngineError, GenerationParams, InferenceProvider},
    Error,
};

/// Provider that connects to a running llama-server or llama.cpp HTTP instance.
/// This is the most practical approach - instead of fighting with direct Rust bindings,
/// we proxy to the HTTP API which is already proven and widely used.
#[derive(Debug)]
pub struct LlamaCppProvider {
    base_url: String,
    default_model: String,
    client: Client,
}

impl LlamaCppProvider {
    pub fn new(base_url: impl Into<String>, default_model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            default_model: default_model.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .expect("Failed to build HTTP client for LlamaCppProvider"),
        }
    }

    /// Health check against the server
    pub async fn health(&self) -> Result<crate::traits::InferenceHealth, anyhow::Error> {
        let resp = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?
            .json::<crate::traits::InferenceHealth>()
            .await?;
        Ok(resp)
    }

    /// Build the OpenAI-chat-completion payload for llama-server
    fn build_payload(&self, req: &traits::ChatRequest) -> serde_json::Value {
        // llama-server expects OpenAI-format messages
        let mut messages = Vec::new();
        for m in &req.messages {
            let mut msg = serde_json::json!({
                "role": m.role,
                "content": m.content,
            });
            // Add tool calls if present (llama-server supports this)
            if !m.tool_calls.is_empty() {
                msg["tool_calls"] = serde_json::Value::Array(
                    m.tool_calls
                        .iter()
                        .map(|tc| {
                            serde_json::json!({
                                "id": tc.id,
                                "type": "function",
                                "function": {
                                    "name": tc.name,
                                    "arguments": tc.arguments.to_string(),
                                }
                            })
                        })
                        .collect(),
                );
            }
            messages.push(msg);
        }

        serde_json::json!({
            "model": &self.default_model,
            "messages": messages,
            "stream": true,
            "temperature": req.params.temperature,
            "top_p": req.params.top_p,
            "max_tokens": req.params.max_tokens,
            "stop": req.params.stop.as_ref().map(|s| s.as_slice()),
        })
    }
}

#[async_trait]
impl traits::InferenceProvider for LlamaCppProvider {
    fn engine_name(&self) -> &str {
        "llama.cpp"
    }

    async fn generate(
        &self,
        prompt: Vec<traits::ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), EngineError> {
        let req = traits::ChatRequest {
            messages: prompt,
            params: params.clone(),
        };

        let payload = self.build_payload(&req);

        // Send streaming request to llama-server
        let mut stream = self
            .client
            .post(&format!("{}/v1/chat/completions", self.base_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| EngineError::GenerationFailed(format!("HTTP request failed: {}", e)))?;

        // Process SSE-style streaming response
        let mut lines = stream.bytes_mut();

        while let Some(result) = lines.next().await {
            let chunk = result.map_err(|e| {
                EngineError::GenerationFailed(format!("Byte read error: {}", e))
            })?;

            let text = String::from_utf8_lossy(&chunk).into_owned();

            // llama-server streams tokens as "data: {...}" SSE lines
            if text.starts_with("data:") {
                let json_str = text.trim_start_matches("data:").trim();
                if json_str == "[DONE]" {
                    break;
                }

                // Try to parse the delta content
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(choice) = data.get("choice") {
                        if let Some(delta) = choice.get("delta") {
                            if let Some(content) = delta.get("content") {
                                if let Some(token) = content.as_str() {
                                    let _ = token_tx.try_send(token.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Send completion marker
        let _ = token_tx.try_send("[DONE]".to_string());

        Ok(())
    }

    async fn unload(&self) -> Result<(), EngineError> {
        // For llama-server, there's typically nothing to "unload" at the provider level
        // The model stays loaded on the server. We just mark as OK.
        Ok(())
    }

    fn capabilities(&self) -> traits::Capabilities {
        traits::Capabilities {
            supports_streaming: true,
            supports_logprobs: false,
            supports_stop_tokens: true,
            max_context_length: 32768, // typical default
        }
    }

    fn provider_name(&self) -> &str {
        "llama.cpp HTTP"
    }
}

/// Adapter that wraps a locally-loaded Llama model (via llama-cpp-2 bindings)
/// for when you have the model in memory and want zero-latency access.
/// Currently a scaffold - full integration would require the llama-cpp-2 Rust crate.
#[derive(Debug)]
pub struct LocalLlamaProvider {
    _model: Option<Llama>, // placeholder - llama-cpp-2 binding
}

#[async_trait]
impl traits::InferenceProvider for LocalLlamaProvider {
    fn engine_name(&self) -> &str {
        "local-llama.cpp"
    }

    async fn generate(
        &self,
        _prompt: Vec<traits::ChatMessage>,
        _params: GenerationParams,
        _token_tx: mpsc::Sender<String>,
    ) -> Result<(), EngineError> {
        Err(EngineError::ModelNotLoaded(
            "LocalLlamaProvider requires llama-cpp-2 bindings not yet configured".into(),
        ))
    }

    async fn unload(&self) -> Result<(), EngineError> {
        // If we had a model, would unload it
        Ok(())
    }

    fn capabilities(&self) -> traits::Capabilities {
        traits::Capabilities {
            supports_streaming: true,
            supports_logprobs: false,
            supports_stop_tokens: true,
            max_context_length: 32768,
        }
    }

    fn provider_name(&self) -> &str {
        "local llama.cpp"
    }
}

impl Default for LlamaCppProvider {
    fn default() -> Self {
        Self::new("http://localhost:8080", "default")
    }
}