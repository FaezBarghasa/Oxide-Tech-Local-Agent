use common::contracts::{InferenceRequest, TaskType};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct SGLangClient {
    pub api_url: String,
    pub model: String,
    pub http: Client,
}

impl SGLangClient {
    pub fn new(api_url: String, model: String) -> Self {
        Self {
            api_url,
            model,
            http: Client::builder()
                .http2_prior_knowledge()
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn generate(
        &self,
        req: &InferenceRequest,
    ) -> Result<serde_json::Value, anyhow::Error> {
        let url = format!("{}/v1/chat/completions", self.api_url);

        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": req.prompt.clone(),
                }
            ],
            "temperature": 0.2,
            "logprobs": true, // We need logprobs for logit/confidence tracking
        });

        let resp = self.http.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            anyhow::bail!("SGLang generation error: {}", resp.status());
        }

        let json_resp: serde_json::Value = resp.json().await?;
        Ok(json_resp)
    }
}

pub struct CloudClient {
    pub http: Client,
}

impl CloudClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub async fn generate(
        &self,
        req: &InferenceRequest,
    ) -> Result<serde_json::Value, anyhow::Error> {
        // Mocking cloud fallback to a cheap/mid/expensive provider (e.g. Anthropic/OpenAI/Groq)
        let url = "https://api.openai.com/v1/chat/completions";
        let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "mock-key".to_string());

        let body = serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": req.prompt.clone(),
                }
            ],
            "temperature": 0.2,
        });

        let resp = self
            .http
            .post(url)
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Cloud fallback generation error: {}", resp.status());
        }

        let json_resp: serde_json::Value = resp.json().await?;
        Ok(json_resp)
    }
}

#[derive(Default)]
pub struct ConfidenceTracker {
    pub weak_confidences: Arc<RwLock<Vec<(String, f32)>>>,
}

impl ConfidenceTracker {
    pub fn new() -> Self {
        Self {
            weak_confidences: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn log_weak(&self, req: &InferenceRequest, confidence: f32) {
        let mut guard = self.weak_confidences.write().await;
        guard.push((req.prompt.clone(), confidence));
        warn!(
            "Logged weak confidence: {} for prompt: {}",
            confidence, req.prompt
        );
    }
}

use crate::moe_router::{ExpertModel, MoeGatingRouter, MoeRoutingDecision};

pub struct LocalFirstRouter {
    pub local_think: SGLangClient,   // Ornith on GPU 0
    pub local_code: SGLangClient,    // Qwen on GPU 1
    pub cloud_fallback: CloudClient, // Last resort only
    pub confidence_tracker: ConfidenceTracker,
    pub moe_router: MoeGatingRouter,
}

impl LocalFirstRouter {
    pub fn new(local_think_url: String, local_code_url: String) -> Self {
        Self {
            local_think: SGLangClient::new(local_think_url, "Ornith-1.5-35B-Q4_K_M".to_string()),
            local_code: SGLangClient::new(
                local_code_url,
                "Qwen3.8-27B-TurboFCFusion-735-882-Here-Uncen-NEO-CODER-MAX-MTP-Q4_K_M".to_string(),
            ),
            cloud_fallback: CloudClient::new(),
            confidence_tracker: ConfidenceTracker::new(),
            moe_router: MoeGatingRouter::new(),
        }
    }

    /// Select optimal expert among Gemma-4-26B-A4B, qwen3.8-27b, Ornith-1.5-35B, and TurboFCFusion
    pub fn route_moe(&self, req: &InferenceRequest) -> MoeRoutingDecision {
        self.moe_router.route(req)
    }

    pub async fn route(&self, req: InferenceRequest) -> Result<serde_json::Value, anyhow::Error> {
        let decision = self.moe_router.route(&req);
        info!("MoE Routing decision: {}", decision.rationale);

        // 1. Dispatch to selected MoE expert
        let local_result = match decision.primary_expert {
            ExpertModel::Ornith1_5_35B_Q4KM => self.local_think.generate(&req).await,
            ExpertModel::Qwen3_8_27B_TurboFCFusion | ExpertModel::Qwen3_8_27B => {
                self.local_code.generate(&req).await
            }
            ExpertModel::Gemma4_26B_A4B => {
                if matches!(req.task_type, TaskType::Training) {
                    return self.cloud_fallback.generate(&req).await;
                }
                self.local_code.generate(&req).await
            }
        };

        // 2. Evaluate quality/confidence
        match local_result {
            Ok(resp) => {
                let confidence = self.calculate_confidence(&resp);
                if confidence >= 0.6 {
                    info!("Local model returned high confidence: {}", confidence);
                    Ok(resp)
                } else {
                    self.confidence_tracker.log_weak(&req, confidence).await;
                    if req.local_failures >= 3 {
                        info!(
                            "Confidence too low ({}) and failures >= 3, calling cloud fallback",
                            confidence
                        );
                        self.cloud_fallback.generate(&req).await
                    } else {
                        // Still return local result but log it
                        Ok(resp)
                    }
                }
            }
            Err(e) => {
                warn!("Local model generation failed: {:?}", e);
                if req.local_failures >= 3 {
                    info!("Local model failed and failures >= 3, calling cloud fallback");
                    self.cloud_fallback.generate(&req).await
                } else {
                    Err(e)
                }
            }
        }
    }

    fn calculate_confidence(&self, resp: &serde_json::Value) -> f32 {
        // Heuristic confidence extraction from logprobs if present, or fallback
        if let Some(choices) = resp.get("choices") {
            if let Some(first_choice) = choices.get(0) {
                if let Some(logprobs) = first_choice.get("logprobs") {
                    if let Some(content_logprobs) = logprobs.get("content") {
                        if let Some(arr) = content_logprobs.as_array() {
                            let mut sum = 0.0;
                            let mut count = 0;
                            for item in arr {
                                if let Some(logprob) = item.get("logprob").and_then(|v| v.as_f64())
                                {
                                    sum += logprob.exp();
                                    count += 1;
                                }
                            }
                            if count > 0 {
                                return (sum / count as f64) as f32;
                            }
                        }
                    }
                }
            }
        }
        0.85 // Default fallback confidence score
    }
}
