use common::contracts::{CloudTrainingSample, UserOutcome, ValidationResult};
use memory::SurrealClient;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

pub struct CloudResponseCapture {
    pub db: Option<Arc<SurrealClient>>,
}

impl CloudResponseCapture {
    pub fn new(db: Option<Arc<SurrealClient>>) -> Self {
        Self { db }
    }

    pub async fn capture(
        &self,
        tenant_id: String,
        user_id: String,
        task_type: String,
        prompt: String,
        cloud_provider: String,
        cloud_response: String,
        cloud_reasoning: Option<String>,
        confidence: f32,
        cost_usd: f32,
    ) -> Result<Uuid, anyhow::Error> {
        let id = Uuid::new_v4();

        // Perform automated validation
        let validation = self.run_validation().await;

        let sample = CloudTrainingSample {
            id,
            tenant_id,
            user_id,
            task_type,
            prompt,
            cloud_provider,
            cloud_response,
            cloud_reasoning,
            validation,
            confidence,
            cost_usd,
            user_outcome: UserOutcome::Accepted, // Default outcome
        };

        if let Some(ref client) = self.db {
            let sample_val = serde_json::to_value(&sample)?;
            let _: Option<serde_json::Value> = client
                .db
                .create(("cloud_training_sample", id.to_string()))
                .content(sample_val)
                .await?;
            info!("Captured cloud training sample with ID: {}", id);
        } else {
            warn!("SurrealDB not initialized; skipping capture storage.");
        }

        Ok(id)
    }

    async fn run_validation(&self) -> ValidationResult {
        // Stubbed validation (normally executing sandbox verifier cargo check / clippy)
        ValidationResult {
            cargo_check: true,
            clippy: true,
            geiger: true,
            qemu: false,
            hardware: None,
        }
    }
}
