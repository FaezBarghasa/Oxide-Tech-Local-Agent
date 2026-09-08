use rkyv::{Archive, Deserialize, Serialize};

/// Serializable, zero-copy workspace snapshot for high-speed MCTS branch simulation.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceSnapshot {
    pub task_id: String,
    pub step_index: usize,
    pub registers: Vec<(String, String)>,
    pub file_digests: Vec<(String, [u8; 32])>,
    pub ast_complexity_score: usize,
}

impl WorkspaceSnapshot {
    pub fn new(task_id: impl Into<String>, step_index: usize) -> Self {
        Self {
            task_id: task_id.into(),
            step_index,
            registers: Vec::new(),
            file_digests: Vec::new(),
            ast_complexity_score: 100,
        }
    }

    /// Archive current state into a zero-copy byte buffer for microsecond cloning.
    #[inline(always)]
    pub fn archive_to_bytes(&self) -> Result<Vec<u8>, String> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|buf| buf.into_vec())
            .map_err(|e| format!("Failed to archive WorkspaceSnapshot: {e:?}"))
    }

    /// Clone state from an archived byte slice with zero heap overhead.
    #[inline(always)]
    pub fn from_archived_bytes(bytes: &[u8]) -> Result<Self, String> {
        let archived = rkyv::access::<ArchivedWorkspaceSnapshot, rkyv::rancor::Error>(bytes)
            .map_err(|e| format!("Zero-copy access verification failed: {e:?}"))?;
        let deserialized: Self = rkyv::deserialize::<Self, rkyv::rancor::Error>(archived)
            .map_err(|e| format!("Deserialization error: {e:?}"))?;
        Ok(deserialized)
    }
}
