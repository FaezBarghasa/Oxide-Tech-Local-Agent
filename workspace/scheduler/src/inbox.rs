//! # Human-in-the-Loop (HITL) Inbox Protocol
//!
//! Stores approval requests for consequential actions (WriteLocal, Exec, External)
//! in SurrealDB and provides asynchronous resumption via Tokio oneshot channels.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::{Surreal, engine::any::Any};
use surrealdb_types::{RecordId, SurrealValue};
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

/// State of an inbox item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SurrealValue)]
pub enum InboxStatus {
    Pending,
    Approved,
    Denied,
    Amended(String),
}

/// An inbox entry persisted to SurrealDB representing an action waiting for human review.
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct InboxEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub inbox_id: Uuid,
    pub dag_id: Uuid,
    pub task_id: String,
    pub reason: String,
    pub risk_class: String,
    pub action_details: String,
    pub status: InboxStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Manager coordinating in-memory parked channels and persisted inbox items.
#[derive(Clone)]
pub struct HitlInboxManager {
    db: Surreal<Any>,
    /// Active parked oneshot channels awaiting resolution
    channels: Arc<Mutex<HashMap<Uuid, oneshot::Sender<bool>>>>,
}

impl HitlInboxManager {
    pub fn new(db: Surreal<Any>) -> Self {
        Self {
            db,
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a pending HITL request and return a receiver channel that awaits human decision.
    pub async fn submit_request(
        &self,
        dag_id: Uuid,
        task_id: &str,
        reason: &str,
        risk_class: &str,
        action_details: &str,
    ) -> Result<(Uuid, oneshot::Receiver<bool>), surrealdb::Error> {
        let inbox_id = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();

        {
            let mut channels = self.channels.lock().await;
            channels.insert(inbox_id, tx);
        }

        let entry = InboxEntry {
            id: None,
            inbox_id,
            dag_id,
            task_id: task_id.to_string(),
            reason: reason.to_string(),
            risk_class: risk_class.to_string(),
            action_details: action_details.to_string(),
            status: InboxStatus::Pending,
            created_at: Utc::now(),
            resolved_at: None,
        };

        let _: Option<InboxEntry> = self.db.create("hitl_inbox").content(entry).await?;
        tracing::info!(%inbox_id, %task_id, "HITL request submitted to inbox");
        Ok((inbox_id, rx))
    }

    /// Resolve a pending HITL item (approved or denied) and notify the parked task.
    pub async fn resolve(
        &self,
        inbox_id: Uuid,
        approved: bool,
    ) -> Result<bool, anyhow::Error> {
        let mut channels = self.channels.lock().await;
        if let Some(tx) = channels.remove(&inbox_id) {
            let status = if approved {
                InboxStatus::Approved
            } else {
                InboxStatus::Denied
            };

            // Update SurrealDB record
            let mut res = self
                .db
                .query("UPDATE hitl_inbox SET status = $status, resolved_at = time::now() WHERE inbox_id = $id")
                .bind(("status", status))
                .bind(("id", inbox_id.to_string()))
                .await?;
            let _updated: Vec<InboxEntry> = res.take(0)?;

            // Signal the parked worker
            let _ = tx.send(approved);
            Ok(true)
        } else {
            anyhow::bail!("No active parked listener found for inbox ID: {inbox_id}")
        }
    }

    /// Retrieve all pending requests from the database.
    pub async fn list_pending(&self) -> Result<Vec<InboxEntry>, surrealdb::Error> {
        let mut response = self
            .db
            .query("SELECT * FROM hitl_inbox WHERE status = 'Pending' ORDER BY created_at ASC")
            .await?;
        let entries: Vec<InboxEntry> = response.take(0)?;
        Ok(entries)
    }
}
