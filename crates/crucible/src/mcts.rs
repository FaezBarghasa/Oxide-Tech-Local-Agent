use crate::shadow_state::WorkspaceSnapshot;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct MctsBranch {
    pub action: String,
    pub visits: usize,
    pub cumulative_reward: f64,
}

impl MctsBranch {
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            visits: 0,
            cumulative_reward: 0.0,
        }
    }

    #[inline(always)]
    pub fn ucb1(&self, total_parent_visits: usize, exploration_constant: f64) -> f64 {
        if self.visits == 0 {
            return f64::MAX;
        }
        let exploitation = self.cumulative_reward / (self.visits as f64);
        let exploration = exploration_constant * ((total_parent_visits as f64).ln() / (self.visits as f64)).sqrt();
        exploitation + exploration
    }
}

pub struct MctsDecisionEngine {
    exploration_constant: f64,
}

impl Default for MctsDecisionEngine {
    fn default() -> Self {
        Self {
            exploration_constant: std::f64::consts::SQRT_2,
        }
    }
}

impl MctsDecisionEngine {
    pub fn new(exploration_constant: f64) -> Self {
        Self { exploration_constant }
    }

    /// Parallel simulation of candidate actions across Rayon threads using zero-copy state branches.
    pub fn evaluate_candidates<F>(
        &self,
        base_state: &WorkspaceSnapshot,
        candidate_actions: &[String],
        evaluation_fn: F,
    ) -> Option<(String, f64)>
    where
        F: Fn(&WorkspaceSnapshot, &str) -> f64 + Sync + Send,
    {
        if candidate_actions.is_empty() {
            return None;
        }

        // Archive once into shared zero-copy byte buffer
        let archived_bytes = match base_state.archive_to_bytes() {
            Ok(b) => b,
            Err(_) => return None,
        };

        // Parallel branch scoring using Rayon
        let evaluated: Vec<(String, f64)> = candidate_actions
            .par_iter()
            .map(|action| {
                // Instantly reconstruct isolated shadow state from archived slice
                if let Ok(shadow_state) = WorkspaceSnapshot::from_archived_bytes(&archived_bytes) {
                    let score = evaluation_fn(&shadow_state, action);
                    (action.clone(), score)
                } else {
                    (action.clone(), f64::MIN)
                }
            })
            .collect();

        evaluated.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}
