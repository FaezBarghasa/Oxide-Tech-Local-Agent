//! # Working Memory
//!
//! Per-agent and per-session scoped memory buffers.
//! Isolates transient scratchpads, sub-agent observations, and partial artifacts
//! so concurrent sub-agents do not cross-contaminate each other's context windows.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::{Surreal, engine::any::Any};
use surrealdb_types::{RecordId, SurrealValue};
use uuid::Uuid;

/// An entry in an agent's working memory buffer.
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct WorkingMemoryEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub session_id: Uuid,
    pub agent_id: String,
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

/// Client for managing scoped working memory partitions in SurrealDB.
#[derive(Clone)]
pub struct WorkingMemoryManager {
    db: Surreal<Any>,
}

impl WorkingMemoryManager {
    pub fn new(db: Surreal<Any>) -> Self {
        Self { db }
    }

    /// Store or update a key-value pair for a specific (session, agent) partition.
    pub async fn set(
        &self,
        session_id: Uuid,
        agent_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), surrealdb::Error> {
        let mut res = self
            .db
            .query(
                "UPSERT working_memory SET value = $val, updated_at = time::now() \
                 WHERE session_id = $session_id AND agent_id = $agent_id AND key = $key",
            )
            .bind(("val", value.to_string()))
            .bind(("session_id", session_id.to_string()))
            .bind(("agent_id", agent_id.to_string()))
            .bind(("key", key.to_string()))
            .await?;

        let _updated: Vec<WorkingMemoryEntry> = res.take(0)?;
        Ok(())
    }

    /// Retrieve a single key from the agent's partition.
    pub async fn get(
        &self,
        session_id: Uuid,
        agent_id: &str,
        key: &str,
    ) -> Result<Option<String>, surrealdb::Error> {
        let mut res = self
            .db
            .query(
                "SELECT value FROM working_memory \
                 WHERE session_id = $session_id AND agent_id = $agent_id AND key = $key LIMIT 1",
            )
            .bind(("session_id", session_id.to_string()))
            .bind(("agent_id", agent_id.to_string()))
            .bind(("key", key.to_string()))
            .await?;

        let entries: Vec<WorkingMemoryEntry> = res.take(0)?;
        Ok(entries.into_iter().next().map(|e| e.value))
    }

    /// Retrieve all keys and values for an agent within the current session.
    pub async fn get_all_for_agent(
        &self,
        session_id: Uuid,
        agent_id: &str,
    ) -> Result<HashMap<String, String>, surrealdb::Error> {
        let mut res = self
            .db
            .query(
                "SELECT * FROM working_memory \
                 WHERE session_id = $session_id AND agent_id = $agent_id",
            )
            .bind(("session_id", session_id.to_string()))
            .bind(("agent_id", agent_id.to_string()))
            .await?;

        let entries: Vec<WorkingMemoryEntry> = res.take(0)?;
        let mut map = HashMap::new();
        for e in entries {
            map.insert(e.key, e.value);
        }
        Ok(map)
    }

    /// Clear all working memory for a completed session.
    pub async fn clear_session(&self, session_id: Uuid) -> Result<usize, surrealdb::Error> {
        let mut res = self
            .db
            .query("DELETE working_memory WHERE session_id = $session_id RETURN BEFORE")
            .bind(("session_id", session_id.to_string()))
            .await?;

        let cleared: Vec<WorkingMemoryEntry> = res.take(0)?;
        Ok(cleared.len())
    }
}
