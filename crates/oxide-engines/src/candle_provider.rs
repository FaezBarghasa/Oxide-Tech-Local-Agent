use async_trait::async_trait;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use tokio::sync::mpsc;
use crate::InferenceProvider;

pub struct CandleProvider {
    model_id: String,
    device_type: String,
}

impl CandleProvider {
    pub fn new(model_id: impl Into<String>, device_type: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            device_type: device_type.into(),
        }
    }
}

#[async_trait]
impl InferenceProvider for CandleProvider {
    fn engine_name(&self) -> &str {
        &self.model_id
    }

    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        _params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> {
        let model_id = self.model_id.clone();
        let device = self.device_type.clone();

        // Offload compute to blocking thread pool to keep async runtime non-blocking
        tokio::task::spawn_blocking(move || {
            tracing::info!("Executing Candle inference on device [{}] for model [{}]", device, model_id);
            // Simulated token generation loop
            let text = format!("[Candle/{}] Response generated for {} messages.", device, prompt.len());
            for word in text.split_whitespace() {
                let chunk = format!(" {}", word);
                if token_tx.blocking_send(chunk).is_err() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        })
        .await
        .map_err(|e| OxideError::Engine(format!("Candle worker task panicked: {}", e)))?;

        Ok(())
    }

    async fn unload(&self) -> Result<(), OxideError> {
        tracing::info!("CandleProvider [{}] unloaded from device [{}].", self.model_id, self.device_type);
        Ok(())
    }
}
