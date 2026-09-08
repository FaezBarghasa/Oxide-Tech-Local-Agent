//! # Agent Journal
//!
//! Durable, event-sourced journal for the multi-agent DAG executor.
//!
//! Every `TaskNode` state transition is **appended** to the `agent_journal` SurrealDB
//! table — never mutated in place. On restart the supervisor replays the journal to
//! restore the exact `TaskDag` state at the last committed boundary, giving crash-safe
//! durable execution comparable to LangGraph's `SqliteSaver` / Restate journal.
//!
//! ## Key Design Choices
//! - **Append-only** — entries are never deleted or updated, only inserted.
//! - **Ordered replay** — `event_seq` (monotonically increasing u64) ensures correct
//!   reconstruction even if wall-clock timestamps collide.
//! - **HITL aware** — `HitlInterrupt` / `HitlResumed` events carry the full decision
//!   payload so the audit trail is complete without additional joins.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::any::Any};
use thiserror::Error;
use uuid::Uuid;

// ── Error Type ────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum JournalError {
    #[error("SurrealDB error: {0}")]
    Db(#[from] surrealdb::Error),
    #[error("Journal replay failed: {0}")]
    Replay(String),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

// ── Domain Types ──────────────────────────────────────────────────────────────

/// Typed artifact reference — replaces the bare `Option<String>` result field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// Human-readable label (e.g. "cargo_check_output", "applied_diff")
    pub label: String,
    /// Filesystem path relative to workspace root, or an inline content key.
    pub path: Option<String>,
    /// Inline small payload (stdout, diff preview, JSON snippet).
    pub inline: Option<String>,
}

/// Rich, typed result for a completed `TaskNode`.
/// Replaces the previous bare `Option<String>` in `supervisor.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// One-sentence summary suitable for the supervisor's context window.
    pub summary: String,
    /// Produced artifacts (diffs, files, shell outputs).
    pub artifacts: Vec<ArtifactRef>,
    /// Prompt + completion tokens consumed by the LLM inference calls.
    pub token_cost: u32,
    /// Wall-clock latency of the entire task from start to completion.
    pub latency_ms: u64,
    /// Agent's own confidence estimate (0.0 – 1.0).
    pub confidence: f32,
    /// Optional quality score injected by the MetaCognitive Observer.
    pub eval_score: Option<f32>,
}

/// Human-in-the-loop decision recorded in the journal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HitlDecision {
    /// Human approved the pending action as proposed.
    Approved,
    /// Human rejected — agent should skip the task and continue.
    Denied,
    /// Human provided amended instructions; the agent should re-plan.
    Amended(String),
}

/// All possible journal events — the complete vocabulary of the agent's execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum JournalEvent {
    /// A new task execution DAG has been created.
    DagCreated {
        dag_id: Uuid,
        goal: String,
        mode: String,
    },
    /// A task node within a DAG has started executing.
    TaskStarted {
        dag_id: Uuid,
        task_id: String,
        role: String,
    },
    /// A task node completed successfully.
    TaskCompleted {
        dag_id: Uuid,
        task_id: String,
        result: TaskResult,
    },
    /// A task node failed — retry_count includes the current attempt.
    TaskFailed {
        dag_id: Uuid,
        task_id: String,
        error: String,
        retry_count: u8,
    },
    /// A task was skipped (dependency chain failure or oscillation guard).
    TaskSkipped {
        dag_id: Uuid,
        task_id: String,
        reason: String,
    },
    /// Git working-tree snapshot created for atomic rollback.
    CheckpointCreated {
        dag_id: Uuid,
        git_sha: String,
    },
    /// Oscillation detected — same error repeated N times.
    OscillationDetected {
        dag_id: Uuid,
        task_id: String,
        error_signature: String,
        count: u8,
    },
    /// Execution paused awaiting human approval.
    HitlInterrupt {
        dag_id: Uuid,
        task_id: String,
        reason: String,
        risk_class: String,
        inbox_id: Uuid,
    },
    /// Human responded — execution may resume.
    HitlResumed {
        dag_id: Uuid,
        task_id: String,
        inbox_id: Uuid,
        decision: HitlDecision,
    },
    /// The entire DAG reached a terminal state.
    DagTerminated {
        dag_id: Uuid,
        outcome: String,
        total_token_cost: u32,
        total_latency_ms: u64,
    },
}

// ── Stored Journal Entry ──────────────────────────────────────────────────────

/// A single immutable row in the `agent_journal` SurrealDB table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Auto-generated SurrealDB record id (populated on read-back).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<surrealdb::RecordId>,
    /// Monotonically increasing sequence within the session (not globally unique).
    pub event_seq: u64,
    /// The concrete event payload.
    pub event: JournalEvent,
    /// Wall-clock timestamp of the insert.
    pub recorded_at: DateTime<Utc>,
}

// ── Journal Client ────────────────────────────────────────────────────────────

const TABLE: &str = "agent_journal";

/// Thin wrapper around `Surreal<Any>` providing type-safe journal operations.
pub struct AgentJournal {
    db: Surreal<Any>,
    /// Monotonic counter — incremented atomically per `append` call within a session.
    seq: std::sync::atomic::AtomicU64,
}

impl AgentJournal {
    /// Wrap an already-connected `Surreal` client.
    pub fn new(db: Surreal<Any>) -> Self {
        Self {
            db,
            seq: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Append a new journal event.
    ///
    /// Returns the `event_seq` assigned to this entry.
    pub async fn append(&self, event: JournalEvent) -> Result<u64, JournalError> {
        let seq = self
            .seq
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let entry = JournalEntry {
            id: None,
            event_seq: seq,
            event,
            recorded_at: Utc::now(),
        };

        let _: Vec<JournalEntry> = self.db.create(TABLE).content(entry).await?;
        tracing::debug!(seq, "agent_journal: appended event");
        Ok(seq)
    }

    /// Replay all journal entries for a given `dag_id` in sequence order.
    ///
    /// Returns entries sorted by `event_seq` ascending — ready for state reconstruction.
    pub async fn replay_dag(&self, dag_id: Uuid) -> Result<Vec<JournalEntry>, JournalError> {
        let dag_str = dag_id.to_string();
        // SurrealQL: select all entries for this dag, ordered by event_seq
        let mut response = self
            .db
            .query(
                "SELECT * FROM agent_journal WHERE event.dag_id = $dag_id ORDER BY event_seq ASC",
            )
            .bind(("dag_id", dag_str))
            .await?;

        let entries: Vec<JournalEntry> = response.take(0)?;
        tracing::info!(dag_id = %dag_id, entries = entries.len(), "agent_journal: replayed dag");
        Ok(entries)
    }

    /// Return the last `N` journal entries across all DAGs (for live monitoring).
    pub async fn tail(&self, n: usize) -> Result<Vec<JournalEntry>, JournalError> {
        let mut response = self
            .db
            .query("SELECT * FROM agent_journal ORDER BY recorded_at DESC LIMIT $n")
            .bind(("n", n))
            .await?;

        let entries: Vec<JournalEntry> = response.take(0)?;
        Ok(entries)
    }

    /// Delete all journal entries for a completed DAG (optional cleanup after archival).
    pub async fn purge_dag(&self, dag_id: Uuid) -> Result<usize, JournalError> {
        let dag_str = dag_id.to_string();
        let mut response = self
            .db
            .query("DELETE agent_journal WHERE event.dag_id = $dag_id RETURN BEFORE")
            .bind(("dag_id", dag_str))
            .await?;

        let purged: Vec<JournalEntry> = response.take(0)?;
        Ok(purged.len())
    }
}

// ── Replay State Reconstructor ────────────────────────────────────────────────

/// Reconstructed DAG state after journal replay.
#[derive(Debug, Default)]
pub struct ReplayedDagState {
    pub dag_id: Uuid,
    pub goal: String,
    pub mode: String,
    /// Map of `task_id` → last known status string ("pending","running","passed","failed","skipped")
    pub task_states: std::collections::HashMap<String, String>,
    /// Map of `task_id` → `TaskResult` for completed tasks
    pub task_results: std::collections::HashMap<String, TaskResult>,
    /// Whether the DAG is waiting for a HITL decision
    pub pending_hitl: Option<(String, Uuid)>, // (task_id, inbox_id)
    /// Whether the DAG has terminated
    pub terminated: bool,
}

impl ReplayedDagState {
    /// Build reconstructed state from a sorted sequence of journal entries.
    pub fn from_entries(entries: &[JournalEntry]) -> Result<Self, JournalError> {
        let mut state = ReplayedDagState::default();

        for entry in entries {
            match &entry.event {
                JournalEvent::DagCreated { dag_id, goal, mode } => {
                    state.dag_id = *dag_id;
                    state.goal = goal.clone();
                    state.mode = mode.clone();
                }
                JournalEvent::TaskStarted { task_id, .. } => {
                    state.task_states.insert(task_id.clone(), "running".into());
                }
                JournalEvent::TaskCompleted { task_id, result, .. } => {
                    state.task_states.insert(task_id.clone(), "passed".into());
                    state.task_results.insert(task_id.clone(), result.clone());
                    // Clear any HITL block if this task just completed
                    if let Some((hitl_task, _)) = &state.pending_hitl {
                        if hitl_task == task_id {
                            state.pending_hitl = None;
                        }
                    }
                }
                JournalEvent::TaskFailed { task_id, .. } => {
                    state.task_states.insert(task_id.clone(), "failed".into());
                }
                JournalEvent::TaskSkipped { task_id, .. } => {
                    state.task_states.insert(task_id.clone(), "skipped".into());
                }
                JournalEvent::HitlInterrupt {
                    task_id, inbox_id, ..
                } => {
                    state.pending_hitl = Some((task_id.clone(), *inbox_id));
                }
                JournalEvent::HitlResumed { .. } => {
                    state.pending_hitl = None;
                }
                JournalEvent::DagTerminated { .. } => {
                    state.terminated = true;
                }
                // Checkpoint and oscillation events don't affect task state reconstruction
                JournalEvent::CheckpointCreated { .. }
                | JournalEvent::OscillationDetected { .. } => {}
            }
        }

        Ok(state)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(summary: &str) -> TaskResult {
        TaskResult {
            summary: summary.to_string(),
            artifacts: vec![],
            token_cost: 100,
            latency_ms: 500,
            confidence: 0.95,
            eval_score: None,
        }
    }

    #[test]
    fn replay_basic_dag_lifecycle() {
        let dag_id = Uuid::new_v4();

        let entries = vec![
            JournalEntry {
                id: None,
                event_seq: 0,
                recorded_at: Utc::now(),
                event: JournalEvent::DagCreated {
                    dag_id,
                    goal: "Implement UART driver".into(),
                    mode: "code".into(),
                },
            },
            JournalEntry {
                id: None,
                event_seq: 1,
                recorded_at: Utc::now(),
                event: JournalEvent::TaskStarted {
                    dag_id,
                    task_id: "plan".into(),
                    role: "Architect".into(),
                },
            },
            JournalEntry {
                id: None,
                event_seq: 2,
                recorded_at: Utc::now(),
                event: JournalEvent::TaskCompleted {
                    dag_id,
                    task_id: "plan".into(),
                    result: make_result("Architecture spec written"),
                },
            },
            JournalEntry {
                id: None,
                event_seq: 3,
                recorded_at: Utc::now(),
                event: JournalEvent::HitlInterrupt {
                    dag_id,
                    task_id: "implement".into(),
                    reason: "WriteLocal action in unattended mode".into(),
                    risk_class: "write_local".into(),
                    inbox_id: Uuid::new_v4(),
                },
            },
        ];

        let state = ReplayedDagState::from_entries(&entries).unwrap();
        assert_eq!(state.dag_id, dag_id);
        assert_eq!(state.task_states["plan"], "passed");
        assert!(state.pending_hitl.is_some());
        assert!(!state.terminated);
    }

    #[test]
    fn task_result_serializes_cleanly() {
        let r = make_result("All tests passed");
        let json = serde_json::to_string(&r).unwrap();
        let back: TaskResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.summary, "All tests passed");
    }

    #[test]
    fn journal_event_tagged_serialization() {
        let event = JournalEvent::DagCreated {
            dag_id: Uuid::nil(),
            goal: "test".into(),
            mode: "code".into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event_type\":\"dag_created\""));
    }
}
