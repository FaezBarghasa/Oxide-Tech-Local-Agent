use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SessionSupervisorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Session already exists and is active: {0}")]
    SessionAlreadyActive(String),
    #[error("Session '{0}' exceeded max recovery attempts (max 1 allowed)")]
    MaxRecoveryExceeded(String),
    #[error("Session '{0}' is terminated in state: {1:?}")]
    SessionTerminated(String, SessionState),
    #[error("Session not found: {0}")]
    NotFound(String),
}

/// Idempotent Agent Session Lifecycle State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Initialized,
    Active,
    Suspended,
    Recovering,
    Closed,
    ProcessLost,
    Failed,
}

/// Fsynced Receipt persisted prior to acknowledging turns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReceipt {
    pub session_id: String,
    pub request_id: String,
    pub state: SessionState,
    pub recovery_attempts: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

impl SessionReceipt {
    pub fn new(session_id: impl Into<String>, request_id: impl Into<String>) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            session_id: session_id.into(),
            request_id: request_id.into(),
            state: SessionState::Initialized,
            recovery_attempts: 0,
            created_at_ms: now,
            updated_at_ms: now,
        }
    }
}

/// Process Supervisor managing idempotent sessions and single-attempt crash recovery
pub struct SessionSupervisor {
    receipt_dir: PathBuf,
    sessions: Arc<DashMap<String, SessionReceipt>>,
    request_to_session: Arc<DashMap<String, String>>,
}

impl SessionSupervisor {
    pub fn new(receipt_dir: impl AsRef<Path>) -> Self {
        let dir = receipt_dir.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&dir);
        Self {
            receipt_dir: dir,
            sessions: Arc::new(DashMap::new()),
            request_to_session: Arc::new(DashMap::new()),
        }
    }

    /// Obtain or create an idempotent session receipt
    pub async fn admit_or_resume_session(
        &self,
        request_id: &str,
    ) -> Result<SessionReceipt, SessionSupervisorError> {
        // 1. Check deduplication by request_id
        if let Some(existing_session_id) = self.request_to_session.get(request_id)
            && let Some(receipt) = self.sessions.get(existing_session_id.value())
        {
            return Ok(receipt.value().clone());
        }

        let session_id = format!("sess-{}", Uuid::now_v7());
        let mut receipt = SessionReceipt::new(&session_id, request_id);
        receipt.state = SessionState::Active;

        self.persist_receipt_sync(&receipt)?;

        self.sessions.insert(session_id.clone(), receipt.clone());
        self.request_to_session
            .insert(request_id.to_string(), session_id);

        Ok(receipt)
    }

    /// Transition session state and enforce single crash recovery attempt
    pub fn transition_state(
        &self,
        session_id: &str,
        new_state: SessionState,
    ) -> Result<SessionReceipt, SessionSupervisorError> {
        let mut entry = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionSupervisorError::NotFound(session_id.to_string()))?;

        if entry.state == SessionState::Closed || entry.state == SessionState::Failed {
            return Err(SessionSupervisorError::SessionTerminated(
                session_id.to_string(),
                entry.state,
            ));
        }

        if new_state == SessionState::Recovering {
            if entry.recovery_attempts >= 1 {
                entry.state = SessionState::Failed;
                entry.updated_at_ms = chrono::Utc::now().timestamp_millis();
                let _ = self.persist_receipt_sync(&entry);
                return Err(SessionSupervisorError::MaxRecoveryExceeded(
                    session_id.to_string(),
                ));
            }
            entry.recovery_attempts += 1;
        }

        entry.state = new_state;
        entry.updated_at_ms = chrono::Utc::now().timestamp_millis();

        let clone = entry.clone();
        drop(entry);

        self.persist_receipt_sync(&clone)?;
        Ok(clone)
    }

    /// Fsync receipt to disk
    fn persist_receipt_sync(&self, receipt: &SessionReceipt) -> Result<(), SessionSupervisorError> {
        let file_path = self
            .receipt_dir
            .join(format!("{}.receipt.json", receipt.session_id));
        let data = serde_json::to_vec_pretty(receipt)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        file.write_all(&data)?;
        file.sync_all()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_idempotent_session_creation_and_recovery_limit() {
        let temp = tempdir().unwrap();
        let supervisor = SessionSupervisor::new(temp.path());

        // 1. Initial admission
        let receipt1 = supervisor.admit_or_resume_session("req-123").await.unwrap();
        assert_eq!(receipt1.state, SessionState::Active);

        // 2. Duplicate request returns same receipt
        let receipt2 = supervisor.admit_or_resume_session("req-123").await.unwrap();
        assert_eq!(receipt1.session_id, receipt2.session_id);

        // 3. First recovery succeeds
        let rec1 = supervisor
            .transition_state(&receipt1.session_id, SessionState::Recovering)
            .unwrap();
        assert_eq!(rec1.recovery_attempts, 1);

        // 4. Second recovery fails (exactly one recovery guarantee)
        let err = supervisor.transition_state(&receipt1.session_id, SessionState::Recovering);
        assert!(err.is_err());
    }
}
