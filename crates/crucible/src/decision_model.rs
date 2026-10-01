//! # Pure-Rust Decision Model & PUCT Monte Carlo Tree Search
//!
//! Replaces random rollouts in Monte Carlo Tree Search with learned policy-value
//! heuristics:
//! - Dual-head evaluation: $\pi(a \mid s)$ tool probability & $V(s) \in [-1.0, 1.0]$ state value.
//! - Dynamic candidate tool pruning: $\pi(\text{action}_k \mid s) < 0.02$ are pruned prior to simulation.
//! - PUCT selection rule:
//!   $$\text{Score}(s, a) = Q(s, a) + c_{\text{puct}} \cdot P(s, a) \cdot \frac{\sqrt{\sum_b N(s, b)}}{1 + N(s, a)}$$

use crate::shadow_state::WorkspaceSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Candidate action with associated prior policy probability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedAction {
    pub name: String,
    pub prior_prob: f64,
    pub value_estimate: f64,
    pub pruned: bool,
}

/// MCTS Node Branch tracking visit counts and cumulative Q-values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuctBranch {
    pub action: String,
    pub prior_p: f64,
    pub visit_count: usize,
    pub total_value: f64,
}

impl PuctBranch {
    pub fn new(action: impl Into<String>, prior_p: f64) -> Self {
        Self {
            action: action.into(),
            prior_p: prior_p.max(0.0),
            visit_count: 0,
            total_value: 0.0,
        }
    }

    #[inline(always)]
    pub fn q_value(&self) -> f64 {
        if self.visit_count == 0 {
            0.0
        } else {
            self.total_value / (self.visit_count as f64)
        }
    }

    /// Calculate PUCT score given total parent visits and exploration constant
    #[inline(always)]
    pub fn puct_score(&self, total_parent_visits: usize, c_puct: f64) -> f64 {
        let q = self.q_value();
        let u = c_puct * self.prior_p * ((total_parent_visits as f64).sqrt() / (1.0 + self.visit_count as f64));
        q + u
    }

    pub fn update(&mut self, reward: f64) {
        self.visit_count += 1;
        self.total_value += reward;
    }
}

/// Policy & Value Evaluator interface for the Decision Model
pub trait PolicyValueModel: Send + Sync {
    /// Evaluates workspace state and candidate actions, returning (prior_probabilities, state_value)
    fn evaluate_state(
        &self,
        state: &WorkspaceSnapshot,
        candidate_actions: &[String],
    ) -> (HashMap<String, f64>, f64);
}

/// Lightweight Pure-Rust Heuristic Decision Model
#[derive(Debug, Clone)]
pub struct HeuristicDecisionModel {
    pub pruning_threshold: f64,
}

impl Default for HeuristicDecisionModel {
    fn default() -> Self {
        Self {
            pruning_threshold: 0.02,
        }
    }
}

impl PolicyValueModel for HeuristicDecisionModel {
    fn evaluate_state(
        &self,
        state: &WorkspaceSnapshot,
        candidate_actions: &[String],
    ) -> (HashMap<String, f64>, f64) {
        if candidate_actions.is_empty() {
            return (HashMap::new(), 0.0);
        }

        let mut priors = HashMap::new();
        let mut unnormalized = Vec::with_capacity(candidate_actions.len());

        // Fast state-action heuristic scoring based on snapshot state
        for action in candidate_actions {
            let act_lower = action.to_lowercase();
            let mut score = 1.0f64;

            if act_lower.contains("test") || act_lower.contains("verify") {
                score += 2.5;
            }
            if act_lower.contains("build") || act_lower.contains("check") {
                score += 2.0;
            }
            if act_lower.contains("inspect") || act_lower.contains("search") {
                score += 1.5;
            }
            if act_lower.contains("format") || act_lower.contains("lint") {
                score += 1.0;
            }
            if act_lower.contains("delete") || act_lower.contains("drop") {
                score *= 0.01; // Severely downweighted / pruned
            }

            unnormalized.push((action.clone(), score));
        }

        let sum_score: f64 = unnormalized.iter().map(|(_, s)| *s).sum();
        let safe_sum = if sum_score > 0.0 { sum_score } else { 1.0 };

        for (action, score) in unnormalized {
            let prob = score / safe_sum;
            priors.insert(action, prob);
        }

        // Value estimate based on state health
        let state_value = if state.file_digests.is_empty() {
            0.8
        } else {
            0.5
        };

        (priors, state_value)
    }
}

/// Decision-Model Guided MCTS Search Engine
pub struct DecisionGuidedMcts<M: PolicyValueModel = HeuristicDecisionModel> {
    pub model: M,
    pub c_puct: f64,
    pub pruning_threshold: f64,
    pub num_simulations: usize,
}

impl Default for DecisionGuidedMcts<HeuristicDecisionModel> {
    fn default() -> Self {
        Self {
            model: HeuristicDecisionModel::default(),
            c_puct: 1.414,
            pruning_threshold: 0.02,
            num_simulations: 50,
        }
    }
}

impl<M: PolicyValueModel> DecisionGuidedMcts<M> {
    pub fn new(model: M, c_puct: f64, pruning_threshold: f64, num_simulations: usize) -> Self {
        Self {
            model,
            c_puct,
            pruning_threshold,
            num_simulations,
        }
    }

    /// Prunes candidates with prior probability < threshold, then simulates PUCT tree search
    pub fn search<F>(
        &self,
        base_state: &WorkspaceSnapshot,
        candidate_actions: &[String],
        rollout_fn: F,
    ) -> Option<(String, f64)>
    where
        F: Fn(&WorkspaceSnapshot, &str) -> f64 + Sync + Send,
    {
        if candidate_actions.is_empty() {
            return None;
        }

        // 1. Evaluate policy & value priors
        let (priors, _v0) = self.model.evaluate_state(base_state, candidate_actions);

        // 2. Dynamic Tool Pruning: Filter out low-confidence branches (< pruning_threshold)
        let active_candidates: Vec<String> = candidate_actions
            .iter()
            .filter(|&a| {
                let p = priors.get(a).copied().unwrap_or(0.0);
                p >= self.pruning_threshold
            })
            .cloned()
            .collect();

        let candidates = if active_candidates.is_empty() {
            candidate_actions.to_vec()
        } else {
            active_candidates
        };

        // 3. Initialize PUCT branches
        let mut branches: Vec<PuctBranch> = candidates
            .iter()
            .map(|a| {
                let p = priors.get(a).copied().unwrap_or(1.0 / candidates.len() as f64);
                PuctBranch::new(a.clone(), p)
            })
            .collect();

        // 4. Run PUCT Simulations
        for _sim in 0..self.num_simulations {
            let total_visits: usize = branches.iter().map(|b| b.visit_count).sum();

            // Select best branch by PUCT score
            let best_idx = branches
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.puct_score(total_visits.max(1), self.c_puct)
                        .partial_cmp(&b.puct_score(total_visits.max(1), self.c_puct))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            let action_to_eval = branches[best_idx].action.clone();
            let reward = rollout_fn(base_state, &action_to_eval);
            branches[best_idx].update(reward);
        }

        // 5. Select best action by highest visit count (most robust decision)
        branches
            .into_iter()
            .max_by_key(|b| b.visit_count)
            .map(|b| {
                let q = b.q_value();
                (b.action, q)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puct_score_calculation() {
        let mut branch = PuctBranch::new("cargo_test", 0.7);
        assert_eq!(branch.visit_count, 0);
        assert_eq!(branch.q_value(), 0.0);

        let initial_puct = branch.puct_score(10, 1.414);
        assert!(initial_puct > 0.0);
        assert!(!initial_puct.is_nan());

        branch.update(1.0);
        assert_eq!(branch.visit_count, 1);
        assert_eq!(branch.q_value(), 1.0);
    }

    #[test]
    fn test_dynamic_tool_pruning() {
        let model = HeuristicDecisionModel::default();
        let state = WorkspaceSnapshot::new("test-task", 0);
        let candidates = vec![
            "cargo_check".to_string(),
            "verify_tests".to_string(),
            "delete_all_files".to_string(),
        ];

        let (priors, value) = model.evaluate_state(&state, &candidates);
        assert!(value > 0.0);
        assert!(priors.get("delete_all_files").copied().unwrap_or(0.0) < 0.02);
        assert!(priors.get("verify_tests").copied().unwrap_or(0.0) > 0.30);
    }

    #[test]
    fn decision_model_mcts_test() {
        let mcts = DecisionGuidedMcts::default();
        let state = WorkspaceSnapshot::new("test-task", 0);
        let candidates = vec![
            "cargo_check".to_string(),
            "cargo_test".to_string(),
            "drop_database".to_string(),
        ];

        let result = mcts.search(&state, &candidates, |_st, act| {
            if act == "cargo_test" {
                1.0
            } else if act == "cargo_check" {
                0.7
            } else {
                -1.0
            }
        });

        assert!(result.is_some());
        let (selected_action, score) = result.unwrap();
        assert_eq!(selected_action, "cargo_test");
        assert!(score > 0.8);
        assert!(!score.is_nan());
    }
}
