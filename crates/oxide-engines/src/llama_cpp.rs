use crate::InferenceProvider;
use async_trait::async_trait;
use futures_util::StreamExt;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use reqwest::Client;
use serde_json::json;
use tokio::sync::mpsc;

/// Provider that connects to a running llama-server or llama.cpp HTTP instance.
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
}

#[async_trait]
impl InferenceProvider for LlamaCppProvider {
    fn engine_name(&self) -> &str {
        &self.name
    }

    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> {
        let endpoint = format!("{}/v1/chat/completions", self.base_url);

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

        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    for line in text.lines() {
                        let line = line.trim();
                        if line.starts_with("data: ") {
                            let data = line.trim_start_matches("data: ").trim();
                            if data == "[DONE]" || data.is_empty() {
                                break;
                            }
                            if let Ok(v) = serde_json::from_str::<serde_json::Value>(data)
                                && let Some(content) = v["choices"][0]["delta"]["content"].as_str()
                                && token_tx.send(content.to_string()).await.is_err()
                            {
                                return Ok(());
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(OxideError::Engine(format!("Stream read error: {}", e)));
                }
            }
        }

        Ok(())
    }

    async fn unload(&self) -> Result<(), OxideError> {
        tracing::info!(
            "LlamaCppProvider {}: unload (no-op, model remains resident).",
            self.name
        );
        Ok(())
    }
}

impl Default for LlamaCppProvider {
    fn default() -> Self {
        Self::new("llama.cpp", "http://localhost:8080")
    }
}
