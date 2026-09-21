use async_trait::async_trait;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use tokio::sync::mpsc;
use crate::InferenceProvider;

pub struct MockProvider {
    name: String,
}

impl MockProvider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}

#[async_trait]
impl InferenceProvider for MockProvider {
    fn engine_name(&self) -> &str {
        &self.name
    }

    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        _params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> {
        let last_user_msg = prompt
            .iter()
            .rev()
            .find(|m| matches!(m.role, oxide_core::Role::User))
            .map(|m| m.text_content())
            .unwrap_or_else(|| "Acknowledged.".to_string());


        let reply = format!("[Oxide-Tech Engine: {}] Processed input: {}", self.name, last_user_msg);
        let tokens: Vec<&str> = reply.split_whitespace().collect();

        for (i, token) in tokens.iter().enumerate() {
            let chunk = if i == 0 {
                token.to_string()
            } else {
                format!(" {}", token)
            };
            if token_tx.send(chunk).await.is_err() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
        }

        Ok(())
    }

    async fn unload(&self) -> Result<(), OxideError> {
        tracing::info!("MockProvider {} unloaded.", self.name);
        Ok(())
    }
}
