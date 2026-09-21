use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tokio::sync::mpsc;

use crate::{InferenceProvider, OxideError};

/// Provider that connects to a running llama-server or llama.cpp HTTP instance.
/// llama-server (https://github.com/ggml-org/llama.cpp)
/// is the most common way to run llama.cpp as a local HTTP service.
#[derive(Debug)]
pub struct LlamaCppProvider {
    name: String,
    base_url: String,
    client: Client,
}

impl LlamaCppProvider {
    pub fn new(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    /// Build the payload for llama-server's OpenAI-compatible endpoint
    fn build_payload(&self, req: &crate::ChatMessageVec, params: &GenerationParams) -> serde_json::Value {
        // llama-server expects OpenAI-format chat messages
        let messages = serde_json::Value::Array(
            req.iter()
                .map(|m| {
                    let mut obj = serde_json::json!({
                        "role": m.role.to_string(),
                        "content": m.content,
                    });
                    // Include tool calls if present (llama-server supports grammar-constrained output via this)
                    obj
                })
                .collect(),
        );

        serde_json::json!({
            "model": &self.name,
            "messages": messages,
            "stream": true,
            "temperature": params.temperature,
            "top_p": params.top_p,
            "max_tokens": params.max_tokens,
            "stop": params.stop.as_ref().map(|s| s.as_slice()),
        })
    }
}

#[async_trait]
impl InferenceProvider for LlamaCppProvider {
    fn engine_name(&self) -> &str {
        "llama.cpp"
    }

    async fn generate(
        &self,
        prompt: Vec<crate::ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> {
        let endpoint = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));

        let body = json!({
            "model": &self.name,
            "messages": prompt,
            "temperature": params.temperature,
            "top_p": params.top_p,
            "max_tokens": params.max_tokens,
            "stream": true,
        });

        let response = self
            .client
            .post(&endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| OxideError::Engine(format!("LLM request failed: {}", e)))?;

        if !response.status().is_success() {
            let err_txt = response.text().await.unwrap_or_default();
            return Err(OxideError::Engine(format!(
                "llama-server returned error: {}",
                err_txt
            )));
        }

        // Process streaming SSE response
        let stream = response.bytes_stream();
        use tokio_stream::StreamExt;

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    for line in text.lines() {
                        let line = line.trim();
                        if line.starts_with("data: ") {
                            let data = line.trim_start_matches("data: ").trim();
                            if data == "[DONE]" || data == "" {
                                break;
                            }
                            if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                                if let Some(content) = v["choices"][0]["delta"]["content"].as_str() {
                                    if token_tx.send(content.to_string()).await.is_err() {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(OxideError::Engine(format!("Stream read error: {}", e)));
                }
            }
        }

        // Send completion marker
        let _ = token_tx.send("[DONE]".to_string()).await;

        Ok(())
    }

    async fn unload(&self) -> Result<(), OxideError> {
        // llama-server typically keeps models loaded in VRAM.
        // No-op unload - the model stays resident.
        tracing::info!("LlamaCppProvider {}: unload (no-op, model remains on server).", self.name);
        Ok(())
    }

    fn capabilities(&self) -> crate::Capabilities {
        crate::Capabilities {
            supports_streaming: true,
            supports_logprobs: false,
            supports_stop_tokens: true,
            max_context_length: 32768,
        }
    }
}

impl Default for LlamaCppProvider {
    fn default() -> Self {
        Self::new("llama.cpp", "http://localhost:8080")
    }
}