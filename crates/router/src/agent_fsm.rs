//! # Agent Finite State Machine (FSM)
//!
//! Enables cyclic dynamic routing inspired by LangGraph conditional edges.
//! Rather than executing a static linear DAG, agents evaluate state after each step
//! and dynamically determine the next role or terminate.

use crate::supervisor::SubAgentRole;
use agent_journal::TaskResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Next routing decision computed dynamically after a task completes or fails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoutingDecision {
    /// Proceed to execute the next sub-agent role with an instruction.
    TransitionTo {
        role: SubAgentRole,
        task_id: String,
        instruction: String,
    },
    /// Re-try the current sub-agent role with diagnostic feedback.
    Retry {
        role: SubAgentRole,
        task_id: String,
        feedback: String,
        attempt: usize,
    },
    /// Loop back to architecture or design reflection due to structural flaws.
    RollbackToPlan { reason: String },
    /// Halt and await human decision (HITL).
    RequestHitl { reason: String, inbox_id: Uuid },
    /// All verification criteria passed; DAG execution succeeds.
    Complete { final_summary: String },
    /// Fatal unrecoverable failure (e.g. oscillation threshold reached).
    FatalError { error: String },
}

/// Dynamic FSM router state machine.
pub struct AgentFsmRouter {
    pub max_retries: usize,
}

impl Default for AgentFsmRouter {
    fn default() -> Self {
        Self { max_retries: 3 }
    }
}

impl AgentFsmRouter {
    pub fn new(max_retries: usize) -> Self {
        Self { max_retries }
    }

    /// Evaluates current execution context and routes to the next state.
    pub fn evaluate_transition(
        &self,
        current_role: SubAgentRole,
        task_id: &str,
        result: Result<&TaskResult, &str>,
        retry_count: usize,
    ) -> RoutingDecision {
        match result {
            Ok(res) => {
                // Check if observer injected a low eval_score requiring revision
                if let Some(score) = res.eval_score {
                    if score < 0.4 {
                        return RoutingDecision::Retry {
                            role: current_role,
                            task_id: task_id.to_string(),
                            feedback: format!(
                                "Quality score too low ({score:.2}). Refine implementation."
                            ),
                            attempt: retry_count + 1,
                        };
                    }
                }

                // Successful transition along the cyclic graph
                match current_role {
                    SubAgentRole::Architect => RoutingDecision::TransitionTo {
                        role: SubAgentRole::Coder,
                        task_id: "implement".to_string(),
                        instruction: format!(
                            "Implement the architecture specification: {}",
                            res.summary
                        ),
                    },
                    SubAgentRole::Coder => RoutingDecision::TransitionTo {
                        role: SubAgentRole::Debugger,
                        task_id: "verify".to_string(),
                        instruction: "Run compiler checks and verify AST integrity.".to_string(),
                    },
                    SubAgentRole::Debugger => RoutingDecision::TransitionTo {
                        role: SubAgentRole::Reviewer,
                        task_id: "security_review".to_string(),
                        instruction: "Perform security and quality audit on verified code."
                            .to_string(),
                    },
                    SubAgentRole::Reviewer => RoutingDecision::Complete {
                        final_summary: format!(
                            "Verification and security audit passed: {}",
                            res.summary
                        ),
                    },
                    SubAgentRole::DevOps => RoutingDecision::Complete {
                        final_summary: format!("DevOps deployment verified: {}", res.summary),
                    },
                }
            }
            Err(err_msg) => {
                if retry_count >= self.max_retries {
                    return RoutingDecision::FatalError {
                        error: format!(
                            "Task '{task_id}' exceeded max retries ({}/{}). Error: {err_msg}",
                            retry_count, self.max_retries
                        ),
                    };
                }

                // If coder failed, route directly to Debugger
                if current_role == SubAgentRole::Coder {
                    RoutingDecision::TransitionTo {
                        role: SubAgentRole::Debugger,
                        task_id: "diagnose_fix".to_string(),
                        instruction: format!(
                            "Analyze compiler diagnostic/error and produce fix diff: {err_msg}"
                        ),
                    }
                } else {
                    // Retry with error message
                    RoutingDecision::Retry {
                        role: current_role,
                        task_id: task_id.to_string(),
                        feedback: err_msg.to_string(),
                        attempt: retry_count + 1,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsm_happy_path_transitions() {
        let fsm = AgentFsmRouter::default();
        let dummy_result = TaskResult {
            summary: "Done".to_string(),
            artifacts: vec![],
            token_cost: 50,
            latency_ms: 100,
            confidence: 0.9,
            eval_score: Some(0.95),
        };

        // Architect -> Coder
        let dec = fsm.evaluate_transition(SubAgentRole::Architect, "arch", Ok(&dummy_result), 0);
        assert!(matches!(
            dec,
            RoutingDecision::TransitionTo {
                role: SubAgentRole::Coder,
                ..
            }
        ));

        // Coder -> Debugger
        let dec2 = fsm.evaluate_transition(SubAgentRole::Coder, "code", Ok(&dummy_result), 0);
        assert!(matches!(
            dec2,
            RoutingDecision::TransitionTo {
                role: SubAgentRole::Debugger,
                ..
            }
        ));

        // Debugger -> Reviewer
        let dec3 = fsm.evaluate_transition(SubAgentRole::Debugger, "verify", Ok(&dummy_result), 0);
        assert!(matches!(
            dec3,
            RoutingDecision::TransitionTo {
                role: SubAgentRole::Reviewer,
                ..
            }
        ));

        // Reviewer -> Complete
        let dec4 = fsm.evaluate_transition(SubAgentRole::Reviewer, "review", Ok(&dummy_result), 0);
        assert!(matches!(dec4, RoutingDecision::Complete { .. }));
    }

    #[test]
    fn test_fsm_error_triggers_debugger_or_retry() {
        let fsm = AgentFsmRouter::new(3);

        // Coder error routes to Debugger
        let dec = fsm.evaluate_transition(
            SubAgentRole::Coder,
            "code",
            Err("mismatched types E0308"),
            0,
        );
        assert!(matches!(
            dec,
            RoutingDecision::TransitionTo {
                role: SubAgentRole::Debugger,
                ..
            }
        ));

        // Debugger error retries up to max_retries
        let dec_retry = fsm.evaluate_transition(
            SubAgentRole::Debugger,
            "verify",
            Err("cargo check failed"),
            1,
        );
        assert!(matches!(
            dec_retry,
            RoutingDecision::Retry { attempt: 2, .. }
        ));

        // Exceeded retries -> FatalError
        let dec_fatal = fsm.evaluate_transition(
            SubAgentRole::Debugger,
            "verify",
            Err("cargo check failed"),
            3,
        );
        assert!(matches!(dec_fatal, RoutingDecision::FatalError { .. }));
    }
}
