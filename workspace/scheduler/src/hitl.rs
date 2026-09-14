use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, oneshot};
use tracing::{info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitlRequest {
    pub id: String,
    pub task_id: String,
    pub tool_name: String,
    pub risk_level: RiskLevel,
    pub description: String,
    pub arguments: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HitlDecision {
    Approved { approved_by: String },
    Rejected { reason: String },
    TimedOut,
}

#[derive(Clone)]
pub struct HitlApprovalChannel {
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<HitlDecision>>>>,
}

impl Default for HitlApprovalChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl HitlApprovalChannel {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.
    pub async fn request_approval(
        &self,
        req: HitlRequest,
        timeout_duration: Duration,
    ) -> Result<HitlDecision> {
        let (tx, rx) = oneshot::channel();
        let req_id = req.id.clone();

        {
            let mut pending = self.pending.lock().await;
            pending.insert(req_id.clone(), tx);
        }

        info!(
            "HITL Interrupt dispatched for request [{}] (Tool: {}, Risk: {:?})",
            req_id, req.tool_name, req.risk_level
        );

        match tokio::time::timeout(timeout_duration, rx).await {
            Ok(Ok(decision)) => {
                info!("HITL Decision received for [{}]: {:?}", req_id, decision);
                Ok(decision)
            }
            Ok(Err(_)) => {
                warn!("HITL channel dropped for request [{}]", req_id);
                Err(anyhow!("HITL approval sender dropped"))
            }
            Err(_) => {
                warn!("HITL approval timed out for request [{}]", req_id);
                let mut pending = self.pending.lock().await;
                pending.remove(&req_id);
                Ok(HitlDecision::TimedOut)
            }
        }
    }

    /// Resolve a pending HITL request with the operator's decision.
    pub async fn submit_decision(&self, req_id: &str, decision: HitlDecision) -> bool {
        let mut pending = self.pending.lock().await;
        if let Some(tx) = pending.remove(req_id) {
            tx.send(decision).is_ok()
        } else {
            false
        }
    }

    /// Check if there are active pending HITL requests.
    pub async fn pending_count(&self) -> usize {
        self.pending.lock().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hitl_approval_flow() {
        let channel = HitlApprovalChannel::new();
        let req = HitlRequest {
            id: "req-123".to_string(),
            task_id: "task-01".to_string(),
            tool_name: "mcp-probe-rs::flash".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Flashing STM32 target hardware".to_string(),
            arguments: serde_json::json!({ "chip": "STM32F407VG" }),
            created_at: Utc::now(),
        };

        let channel_clone = channel.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            channel_clone
                .submit_decision(
                    "req-123",
                    HitlDecision::Approved {
                        approved_by: "lead_systems_engineer".to_string(),
                    },
                )
                .await;
        });

        let decision = channel
            .request_approval(req, Duration::from_secs(2))
            .await
            .unwrap();

        match decision {
            HitlDecision::Approved { approved_by } => {
                assert_eq!(approved_by, "lead_systems_engineer");
            }
            _ => panic!("Expected approval decision"),
        }
    }
}
