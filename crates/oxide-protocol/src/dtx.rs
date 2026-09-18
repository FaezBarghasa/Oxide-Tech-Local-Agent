use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Time-ordered UUIDv7 Distributed Transaction ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DtxId(pub Uuid);

impl DtxId {
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for DtxId {
    fn default() -> Self {
        Self::new_v7()
    }
}

impl std::fmt::Display for DtxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session & Tenant identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionKey {
    pub user_id: String,
    pub session_id: Uuid,
    pub namespace: String,
}

impl SessionKey {
    pub fn new(user_id: impl Into<String>) -> Self {
        let user_id = user_id.into();
        let hash = blake3::hash(user_id.as_bytes()).to_hex();
        let namespace = format!("u_{}", &hash[..12]);
        Self {
            user_id,
            session_id: Uuid::now_v7(),
            namespace,
        }
    }
}

/// Distributed Transaction Envelope carrying causal trace and tenant boundary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtxEnvelope {
    pub dtx_id: DtxId,
    pub causation_id: Option<DtxId>,
    pub session: SessionKey,
    pub op: DtxOp,
    pub deadline_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceRef {
    pub domain: String,
    pub resource_uri: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    Commit,
    Abort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortReason {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationAction {
    pub action_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub artifact_id: Uuid,
    pub uri: String,
    pub blake3_hash: String,
}

/// Operation payload in a Distributed Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DtxOp {
    Begin {
        participants: Vec<ResourceRef>,
    },
    Prepare {
        participant: ResourceRef,
        vote: Vote,
    },
    Commit {
        participant: ResourceRef,
    },
    Abort {
        participant: ResourceRef,
        reason: AbortReason,
    },
    Compensate {
        participant: ResourceRef,
        action: CompensationAction,
    },
    IrreversibleAck {
        participant: ResourceRef,
        backup_ref: ArtifactRef,
    },
}

/// Every resource participating in a Distributed Transaction implements this trait.
#[async_trait::async_trait]
pub trait ResourceTx: Send + Sync {
    async fn prepare(&self, env: &DtxEnvelope) -> Result<Vote, String>;
    async fn commit(&self, env: &DtxEnvelope) -> Result<(), String>;
    async fn rollback(&self, env: &DtxEnvelope) -> Result<(), String>;
    /// Hardware/flash/fuse operations return true here.
    fn is_irreversible(&self, env: &DtxEnvelope) -> bool;
    /// For irreversible operations: capture a compensation artifact BEFORE commit.
    async fn snapshot_for_compensation(&self, env: &DtxEnvelope) -> Result<ArtifactRef, String>;
}
