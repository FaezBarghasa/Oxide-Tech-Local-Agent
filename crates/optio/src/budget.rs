use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Budget configuration and trackers for token and tool execution governance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    /// Maximum prompt + completion tokens allowed per individual task turn.
    pub max_tokens_per_task: usize,
    /// Maximum cumulative tokens allowed per overall session.
    pub max_tokens_per_session: usize,
    /// Maximum tool invocations allowed per task before requiring HITL confirmation or aborting.
    pub max_tool_calls_per_task: usize,
    /// Maximum consecutive failed tool calls before flagging an oscillation / error state.
    pub max_consecutive_tool_failures: usize,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            max_tokens_per_task: 16_384,
            max_tokens_per_session: 131_072,
            max_tool_calls_per_task: 30,
            max_consecutive_tool_failures: 3,
        }
    }
}

/// Dynamic tracker for token and tool call consumption.
#[derive(Debug, Clone)]
pub struct BudgetTracker {
    pub config: BudgetConfig,
    session_tokens_used: Arc<AtomicUsize>,
    task_tokens_used: Arc<AtomicUsize>,
    task_tool_calls: Arc<AtomicUsize>,
    consecutive_tool_failures: Arc<AtomicUsize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetViolation {
    TaskTokenLimitExceeded { used: usize, limit: usize },
    SessionTokenLimitExceeded { used: usize, limit: usize },
    TaskToolCallLimitExceeded { used: usize, limit: usize },
    ConsecutiveToolFailuresExceeded { count: usize, limit: usize },
}

impl std::fmt::Display for BudgetViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetViolation::TaskTokenLimitExceeded { used, limit } => {
                write!(f, "Task token limit exceeded: {} / {}", used, limit)
            }
            BudgetViolation::SessionTokenLimitExceeded { used, limit } => {
                write!(f, "Session token limit exceeded: {} / {}", used, limit)
            }
            BudgetViolation::TaskToolCallLimitExceeded { used, limit } => {
                write!(f, "Task tool call limit exceeded: {} / {}", used, limit)
            }
            BudgetViolation::ConsecutiveToolFailuresExceeded { count, limit } => {
                write!(f, "Consecutive tool failures exceeded: {} / {}", count, limit)
            }
        }
    }
}

impl std::error::Error for BudgetViolation {}

impl BudgetTracker {
    pub fn new(config: BudgetConfig) -> Self {
        Self {
            config,
            session_tokens_used: Arc::new(AtomicUsize::new(0)),
            task_tokens_used: Arc::new(AtomicUsize::new(0)),
            task_tool_calls: Arc::new(AtomicUsize::new(0)),
            consecutive_tool_failures: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Reset per-task counters when starting a new task node.
    pub fn reset_task(&self) {
        self.task_tokens_used.store(0, Ordering::SeqCst);
        self.task_tool_calls.store(0, Ordering::SeqCst);
        self.consecutive_tool_failures.store(0, Ordering::SeqCst);
    }

    /// Record tokens used and check budget thresholds.
    pub fn record_tokens(&self, tokens: usize) -> Result<(), BudgetViolation> {
        let task_curr = self.task_tokens_used.fetch_add(tokens, Ordering::SeqCst) + tokens;
        let session_curr = self.session_tokens_used.fetch_add(tokens, Ordering::SeqCst) + tokens;

        if task_curr > self.config.max_tokens_per_task {
            return Err(BudgetViolation::TaskTokenLimitExceeded {
                used: task_curr,
                limit: self.config.max_tokens_per_task,
            });
        }

        if session_curr > self.config.max_tokens_per_session {
            return Err(BudgetViolation::SessionTokenLimitExceeded {
                used: session_curr,
                limit: self.config.max_tokens_per_session,
            });
        }

        Ok(())
    }

    /// Record a tool invocation and success/failure status.
    pub fn record_tool_call(&self, success: bool) -> Result<(), BudgetViolation> {
        let calls = self.task_tool_calls.fetch_add(1, Ordering::SeqCst) + 1;
        if calls > self.config.max_tool_calls_per_task {
            return Err(BudgetViolation::TaskToolCallLimitExceeded {
                used: calls,
                limit: self.config.max_tool_calls_per_task,
            });
        }

        if success {
            self.consecutive_tool_failures.store(0, Ordering::SeqCst);
        } else {
            let failures = self.consecutive_tool_failures.fetch_add(1, Ordering::SeqCst) + 1;
            if failures >= self.config.max_consecutive_tool_failures {
                return Err(BudgetViolation::ConsecutiveToolFailuresExceeded {
                    count: failures,
                    limit: self.config.max_consecutive_tool_failures,
                });
            }
        }

        Ok(())
    }

    pub fn get_session_tokens_used(&self) -> usize {
        self.session_tokens_used.load(Ordering::SeqCst)
    }

    pub fn get_task_tokens_used(&self) -> usize {
        self.task_tokens_used.load(Ordering::SeqCst)
    }

    pub fn get_task_tool_calls(&self) -> usize {
        self.task_tool_calls.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_token_limits() {
        let config = BudgetConfig {
            max_tokens_per_task: 100,
            max_tokens_per_session: 150,
            max_tool_calls_per_task: 5,
            max_consecutive_tool_failures: 2,
        };
        let tracker = BudgetTracker::new(config);

        assert!(tracker.record_tokens(50).is_ok());
        assert_eq!(tracker.get_task_tokens_used(), 50);

        // Exceed task tokens
        let err = tracker.record_tokens(60);
        assert!(matches!(err, Err(BudgetViolation::TaskTokenLimitExceeded { .. })));

        // Reset task and test session limits
        tracker.reset_task();
        assert_eq!(tracker.get_task_tokens_used(), 0);
        assert_eq!(tracker.get_session_tokens_used(), 110);

        let err2 = tracker.record_tokens(50);
        assert!(matches!(err2, Err(BudgetViolation::SessionTokenLimitExceeded { .. })));
    }

    #[test]
    fn test_budget_tool_call_failures() {
        let config = BudgetConfig {
            max_tokens_per_task: 1000,
            max_tokens_per_session: 5000,
            max_tool_calls_per_task: 10,
            max_consecutive_tool_failures: 2,
        };
        let tracker = BudgetTracker::new(config);

        assert!(tracker.record_tool_call(false).is_ok());
        let err = tracker.record_tool_call(false);
        assert!(matches!(err, Err(BudgetViolation::ConsecutiveToolFailuresExceeded { .. })));
    }
}
