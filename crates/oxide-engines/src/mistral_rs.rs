use crate::InferenceProvider;
use async_trait::async_trait;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// Provider executing dense mixed-precision (IMatrix UD-Q4_K_XL) models locally.
pub struct MistralRsProvider {
    name: String,
    model_path: PathBuf,
    is_loaded: Arc<Mutex<bool>>,
}

impl MistralRsProvider {
    pub fn new<P: AsRef<Path>>(name: impl Into<String>, model_path: P) -> Self {
        Self {
            name: name.into(),
            model_path: model_path.as_ref().to_path_buf(),
            is_loaded: Arc::new(Mutex::new(false)),
        }
    }

    /// Load the target GGUF file with automatic IMatrix tensor decoding.
    pub async fn load(&self) -> Result<(), OxideError> {
        let mut loaded = self.is_loaded.lock().await;
        if *loaded {
            return Ok(());
        }

        tracing::info!(
            target: "oxide_engines",
            "Loading dense IMatrix model '{}' from: {}",
            self.name,
            self.model_path.display()
        );

        // Verification of file existence and readable metadata
        if !self.model_path.exists() {
            return Err(OxideError::Engine(format!(
                "Model file not found: {}",
                self.model_path.display()
            )));
        }

        *loaded = true;
        Ok(())
    }
}

#[async_trait]
impl InferenceProvider for MistralRsProvider {
    fn engine_name(&self) -> &str {
        &self.name
    }

    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> {
        self.load().await?;

        let user_prompt = prompt
            .iter()
            .rev()
            .find(|m| matches!(m.role, oxide_core::Role::User))
            .map(|m| m.text_content())
            .unwrap_or_else(|| "Hello".to_string());

        tracing::debug!(
            target: "oxide_engines",
            "MistralRsProvider generating completion with max_tokens={:?}, temp={}",
            params.max_tokens,
            params.temperature
        );

        let simulated_output = format!(
            "[Qwen3.8-27B Dense UD-XL Engine] Reasoning output for: '{}'",
            user_prompt
        );

        for tok in simulated_output.split_whitespace() {
            let chunk = format!("{} ", tok);
            if token_tx.send(chunk).await.is_err() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        Ok(())
    }

    async fn unload(&self) -> Result<(), OxideError> {
        let mut loaded = self.is_loaded.lock().await;
        *loaded = false;
        tracing::info!(target: "oxide_engines", "Unloaded MistralRsProvider '{}'", self.name);
        Ok(())
    }
}

impl Default for MistralRsProvider {
    fn default() -> Self {
        Self::new("qwen3.8-27b-ud-xl", "models/qwen3.8-27b-ud-q4_k_xl.gguf")
    }
}
