use crate::dtx::DtxId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationDelta {
    Pass,
    Fail,
    Warning,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofKind {
    KaniFormalProof,
    BoundedModelCheck,
    DifferentialFuzz { iterations: u64, match_rate: f64 },
    UnitRegression,
    Advisory { judge: String, reasoning: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProof {
    pub proof_id: Uuid,
    pub kind: ProofKind,
    pub is_formal: bool, // True for Kani/BMC/Fuzz, False for Advisory
    pub passed: bool,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: Uuid,
    pub dtx_id: DtxId,
    pub inputs_hash: String,
    pub tool_versions: std::collections::HashMap<String, String>,
    pub proofs: Vec<VerificationProof>,
    pub delta: VerificationDelta,
    pub generated_at: DateTime<Utc>,
    pub ed25519_signature: Option<String>,
}

impl EvidenceBundle {
    pub fn new(dtx_id: DtxId, inputs_hash: impl Into<String>) -> Self {
        Self {
            bundle_id: Uuid::now_v7(),
            dtx_id,
            inputs_hash: inputs_hash.into(),
            tool_versions: std::collections::HashMap::new(),
            proofs: Vec::new(),
            delta: VerificationDelta::Pass,
            generated_at: Utc::now(),
            ed25519_signature: None,
        }
    }

    pub fn compute_content_hash(&self) -> String {
        let serialized = serde_json::to_vec(self).unwrap_or_default();
        blake3::hash(&serialized).to_hex().to_string()
    }
}
