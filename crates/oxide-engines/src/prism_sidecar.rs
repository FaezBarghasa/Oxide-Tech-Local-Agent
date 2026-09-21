use crate::InferenceProvider;
use async_trait::async_trait;
use futures_util::StreamExt;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use reqwest::Client;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, mpsc};

/// Provider that spawns and supervises a dedicated PrismML llama.cpp sidecar for Ternary-Bonsai.
pub struct PrismBonsaiEngine {
    name: String,
    base_url: String,
    port: u16,
    child: Arc<Mutex<Option<Child>>>,
    client: Client,
}

impl PrismBonsaiEngine {
    /// Locate the prism-llama-server binary from environment or local search paths.
    pub fn get_sidecar_path(binary_name: &str) -> PathBuf {
        if let Ok(env_path) = std::env::var("PRISM_LLAMA_SERVER_BIN") {
            return PathBuf::from(env_path);
        }

        let candidates = [
            format!("/usr/local/bin/{}", binary_name),
            format!("/usr/bin/{}", binary_name),
            format!("./bin/{}", binary_name),
            format!("./target/release/{}", binary_name),
        ];

        for c in &candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return p;
            }
        }

        PathBuf::from(binary_name)
    }

    /// Spawn a new dedicated PrismML Bonsai llama.cpp sidecar with Flash Attention and GPU offload.
    pub async fn spawn<P: AsRef<Path>>(
        model_path: P,
        port: u16,
        gpu_layers: u32,
        context_len: usize,
    ) -> Result<Self, OxideError> {
        let bin = Self::get_sidecar_path("prism-llama-server");
        let model_str = model_path.as_ref().to_string_lossy().to_string();

        tracing::info!(
            target: "oxide_engines",
            "Spawning Prism Bonsai sidecar on port {} with model: {}",
            port,
            model_str
        );

        let child = Command::new(&bin)
            .args([
                "-m",
                &model_str,
                "--port",
                &port.to_string(),
                "-ngl",
                &gpu_layers.to_string(),
                "-fa",
                "on",
                "-c",
                &context_len.to_string(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                OxideError::Engine(format!(
                    "Failed to spawn prism-llama-server at '{}': {}",
                    bin.display(),
                    e
                ))
            })?;

        let base_url = format!("http://127.0.0.1:{}", port);

        Ok(Self {
            name: "ternary-bonsai".to_string(),
            base_url,
            port,
            child: Arc::new(Mutex::new(Some(child))),
            client: Client::new(),
        })
    }

    /// Create an engine handle connected to an already running Prism sidecar.
    pub fn connect(base_url: impl Into<String>, port: u16) -> Self {
        Self {
            name: "ternary-bonsai".to_string(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            port,
            child: Arc::new(Mutex::new(None)),
            client: Client::new(),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

#[async_trait]
impl InferenceProvider for PrismBonsaiEngine {
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
            "top_k": params.top_k,
            "min_p": params.min_p,
            "presence_penalty": params.presence_penalty,
            "repetition_penalty": params.repetition_penalty,
            "max_tokens": params.max_tokens,
            "stream": true,
        });

        let response = self
            .client
            .post(&endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                OxideError::Engine(format!("Prism Bonsai sidecar request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let err_txt = response.text().await.unwrap_or_default();
            return Err(OxideError::Engine(format!(
                "prism-llama-server returned error: {}",
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
        let mut lock = self.child.lock().await;
        if let Some(mut child) = lock.take() {
            tracing::info!(target: "oxide_engines", "Killing Prism Bonsai sidecar process (pid={:?})", child.id());
            let _ = child.kill().await;
        }
        Ok(())
    }
}
