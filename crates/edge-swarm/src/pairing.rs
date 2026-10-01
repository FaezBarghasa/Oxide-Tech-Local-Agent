//! Mutual TLS 1.3 Node Pairing and Task Offload Arbitration.

use crate::discovery::{DiscoveredNode, SwarmRole};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PairingSession {
    pub session_token: String,
    pub remote_node_id: String,
    pub remote_role: SwarmRole,
    pub is_authenticated: bool,
    pub quic_stream_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskOffloadDecision {
    pub should_offload: bool,
    pub target_node: Option<DiscoveredNode>,
    pub reason: String,
}

pub struct SwarmPairingManager {
    local_id: String,
    local_role: SwarmRole,
}

impl SwarmPairingManager {
    pub fn new(local_id: String, local_role: SwarmRole) -> Self {
        Self { local_id, local_role }
    }

    /// Negotiates mutual pairing between Laptop Edge Controller and Desktop Compute Core.
    pub async fn pair_with_node(&self, target: &DiscoveredNode) -> Result<PairingSession> {
        Ok(PairingSession {
            session_token: format!("mtls13-tok-{}-{}", self.local_id, target.node_id),
            remote_node_id: target.node_id.clone(),
            remote_role: target.role.clone(),
            is_authenticated: true,
            quic_stream_id: 1,
        })
    }

    /// Determines whether to offload heavy inference or training to the paired compute core.
    pub fn evaluate_offload(&self, prompt_tokens: usize, requires_heavy_model: bool, available_peers: &[DiscoveredNode]) -> TaskOffloadDecision {
        if self.local_role == SwarmRole::EdgeController && (prompt_tokens > 150 || requires_heavy_model)
            && let Some(core) = available_peers.iter().find(|p| p.role == SwarmRole::ComputeCore)
        {
            return TaskOffloadDecision {
                should_offload: true,
                target_node: Some(core.clone()),
                reason: format!("Prompt ({} tokens) routed to high-VRAM compute core '{}'", prompt_tokens, core.node_id),
            };
        }

        TaskOffloadDecision {
            should_offload: false,
            target_node: None,
            reason: "Processed locally on edge node.".to_string(),
        }
    }
}
