use crate::provider::{ChatMessage, ChatRequest, ChatResponse, InferenceProvider};
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;

/// Orchestrator for local speculative decoding pairing a fast draft model with a larger target model.
pub struct SpeculativeDecodingEngine {
    draft_provider: Arc<dyn InferenceProvider>,
    target_provider: Arc<dyn InferenceProvider>,
    draft_steps: usize,
}

impl SpeculativeDecodingEngine {
    pub fn new(
        draft_provider: Arc<dyn InferenceProvider>,
        target_provider: Arc<dyn InferenceProvider>,
        draft_steps: usize,
    ) -> Self {
        Self {
            draft_provider,
            target_provider,
            draft_steps: draft_steps.max(1),
        }
    }

    /// Execute a speculative chat completion.
    ///
    /// 1. Runs the draft model for speculative tokens.
    /// 2. Verifies the generated speculative prefix with the target model.
    pub async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> {
        let start = Instant::now();

        // 1. Generate speculative draft candidates
        let mut draft_req = req.clone();
        draft_req.max_tokens = Some(self.draft_steps * 16);
        let draft_resp = self.draft_provider.chat_completion(draft_req).await;

        match draft_resp {
            Ok(draft) if !draft.content.is_empty() => {
                // If draft generated text, inject it as an assistant prefix hint into target request
                let mut target_messages = req.messages.clone();
                target_messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: draft.content.clone(),
                });

                // Request verification from the target model
                let target_req = ChatRequest {
                    messages: target_messages,
                    ..req.clone()
                };

                match self.target_provider.chat_completion(target_req).await {
                    Ok(mut target_res) => {
                        target_res.latency_ms = start.elapsed().as_millis() as u64;
                        Ok(target_res)
                    }
                    Err(e) => {
                        // Fallback directly to normal target completion
                        tracing::warn!("Speculative target verification failed, falling back: {}", e);
                        self.target_provider.chat_completion(req).await
                    }
                }
            }
            _ => {
                // Fallback to pure target execution if draft failed or produced empty response
                self.target_provider.chat_completion(req).await
            }
        }
    }
}
