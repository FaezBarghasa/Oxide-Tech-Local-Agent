use crate::OxideError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::watch;

/// Comprehensive state enum for the agent loop lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "payload")]
pub enum AgentState {
    /// Agent is idle waiting for incoming requests.
    Idle,
    /// Context ingestion and prompt evaluation in progress.
    Thinking { context_len: usize },
    /// Agent is invoking an external tool or WASM module.
    ToolCalling { tool_name: String, call_id: String },
    /// Streaming token responses back to the client.
    Streaming { tokens_emitted: usize },
    /// Turn finished successfully.
    Halted { reason: String },
    /// Kernel, FFI, or execution fault encountered.
    Fault { code: u32, message: String },
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Idle => write!(f, "IDLE"),
            AgentState::Thinking { context_len } => write!(f, "THINKING(ctx={})", context_len),
            AgentState::ToolCalling { tool_name, call_id } => {
                write!(f, "TOOL_CALLING({}:{})", tool_name, call_id)
            }
            AgentState::Streaming { tokens_emitted } => {
                write!(f, "STREAMING(tokens={})", tokens_emitted)
            }
            AgentState::Halted { reason } => write!(f, "HALTED({})", reason),
            AgentState::Fault { code, message } => write!(f, "FAULT({}: {})", code, message),
        }
    }
}

/// Recorded state transition with timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: AgentState,
    pub to: AgentState,
    pub timestamp_ms: i64,
}

/// Pinned, observable agent state machine driver.
pub struct AgentStateMachine {
    current_state: AgentState,
    state_tx: watch::Sender<AgentState>,
    state_rx: watch::Receiver<AgentState>,
    transition_count: Arc<AtomicUsize>,
}

impl Default for AgentStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentStateMachine {
    /// Create a new state machine initialized in `Idle`.
    pub fn new() -> Self {
        let initial = AgentState::Idle;
        let (state_tx, state_rx) = watch::channel(initial.clone());
        Self {
            current_state: initial,
            state_tx,
            state_rx,
            transition_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Read the current state directly.
    pub fn current(&self) -> &AgentState {
        &self.current_state
    }

    /// Obtain a lock-free watch receiver for downstream state subscribers.
    pub fn subscribe(&self) -> watch::Receiver<AgentState> {
        self.state_rx.clone()
    }

    /// Total valid transitions executed.
    pub fn total_transitions(&self) -> usize {
        self.transition_count.load(Ordering::Relaxed)
    }

    /// Attempt a deterministic state transition, validating legal state paths.
    pub fn transition_to(&mut self, next: AgentState) -> Result<StateTransition, OxideError> {
        self.validate_transition(&self.current_state, &next)?;

        let prev = std::mem::replace(&mut self.current_state, next.clone());
        self.transition_count.fetch_add(1, Ordering::Relaxed);
        let _ = self.state_tx.send(next.clone());

        Ok(StateTransition {
            from: prev,
            to: next,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        })
    }

    /// Transition directly to Fault state from any active state.
    pub fn fault(&mut self, code: u32, message: impl Into<String>) -> StateTransition {
        let next = AgentState::Fault {
            code,
            message: message.into(),
        };
        let prev = std::mem::replace(&mut self.current_state, next.clone());
        self.transition_count.fetch_add(1, Ordering::Relaxed);
        let _ = self.state_tx.send(next.clone());

        StateTransition {
            from: prev,
            to: next,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Enforce valid state machine transitions according to deterministic lifecycle rules.
    fn validate_transition(&self, from: &AgentState, to: &AgentState) -> Result<(), OxideError> {
        match (from, to) {
            // Can always transition to Fault
            (_, AgentState::Fault { .. }) => Ok(()),

            // Idle transitions
            (AgentState::Idle, AgentState::Thinking { .. }) => Ok(()),
            (AgentState::Idle, AgentState::ToolCalling { .. }) => Ok(()),

            // Thinking transitions
            (AgentState::Thinking { .. }, AgentState::Streaming { .. }) => Ok(()),
            (AgentState::Thinking { .. }, AgentState::ToolCalling { .. }) => Ok(()),
            (AgentState::Thinking { .. }, AgentState::Halted { .. }) => Ok(()),

            // ToolCalling transitions
            (AgentState::ToolCalling { .. }, AgentState::Thinking { .. }) => Ok(()),
            (AgentState::ToolCalling { .. }, AgentState::Streaming { .. }) => Ok(()),
            (AgentState::ToolCalling { .. }, AgentState::Halted { .. }) => Ok(()),

            // Streaming transitions
            (AgentState::Streaming { .. }, AgentState::Streaming { .. }) => Ok(()),
            (AgentState::Streaming { .. }, AgentState::ToolCalling { .. }) => Ok(()),
            (AgentState::Streaming { .. }, AgentState::Halted { .. }) => Ok(()),

            // Halted / Fault can reset to Idle
            (AgentState::Halted { .. }, AgentState::Idle) => Ok(()),
            (AgentState::Fault { .. }, AgentState::Idle) => Ok(()),

            // All other transitions are invalid
            _ => Err(OxideError::Runtime(format!(
                "Illegal agent state transition from {} to {}",
                from, to
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_lifecycle_transitions() {
        let mut sm = AgentStateMachine::new();
        assert_eq!(*sm.current(), AgentState::Idle);

        assert!(sm
            .transition_to(AgentState::Thinking { context_len: 2048 })
            .is_ok());
        assert!(sm
            .transition_to(AgentState::Streaming { tokens_emitted: 1 })
            .is_ok());
        assert!(sm
            .transition_to(AgentState::Streaming { tokens_emitted: 2 })
            .is_ok());
        assert!(sm
            .transition_to(AgentState::Halted {
                reason: "stop".into()
            })
            .is_ok());
        assert!(sm.transition_to(AgentState::Idle).is_ok());
        assert_eq!(sm.total_transitions(), 5);
    }

    #[test]
    fn test_illegal_transition_rejection() {
        let mut sm = AgentStateMachine::new();
        // Cannot jump directly from Idle to Halted without thinking or tool calling
        let res = sm.transition_to(AgentState::Halted {
            reason: "invalid".into(),
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_fault_isolation() {
        let mut sm = AgentStateMachine::new();
        sm.transition_to(AgentState::Thinking { context_len: 512 })
            .unwrap();
        let tr = sm.fault(500, "Kernel driver panic");
        assert_eq!(
            tr.to,
            AgentState::Fault {
                code: 500,
                message: "Kernel driver panic".into()
            }
        );
        // Can recover back to Idle
        assert!(sm.transition_to(AgentState::Idle).is_ok());
    }
}
