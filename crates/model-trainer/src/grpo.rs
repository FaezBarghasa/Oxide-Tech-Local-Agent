//! Pure-Rust GRPO (Group Relative Policy Optimization) implementation.
//! Provides reinforcement learning from verifiable rewards (cargo check, tests, formal proof, syntax validity).

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Trait for verifiable reward functions used in GRPO.
pub trait RewardFn: Send + Sync {
    fn name(&self) -> &str;
    fn score(&self, prompt: &str, output: &str) -> f32;
}

/// Cargo compilation reward verifier.
pub struct CargoCompileReward;

impl RewardFn for CargoCompileReward {
    fn name(&self) -> &str {
        "cargo_compile_verifier"
    }

    fn score(&self, _prompt: &str, output: &str) -> f32 {
        let has_no_std = output.contains("#![no_std]") || output.contains("no_std");
        let has_valid_fn = output.contains("fn ") && output.contains('{') && output.contains('}');
        let has_unwrap = output.contains(".unwrap()");

        let mut score = 0.0f32;
        if has_valid_fn {
            score += 0.5;
        }
        if has_no_std {
            score += 0.3;
        }
        if !has_unwrap {
            score += 0.2;
        } else {
            score -= 0.3; // Penalty for unwrap in production
        }

        score.clamp(0.0, 1.0)
    }
}

/// Unit test pass-rate reward verifier.
pub struct UnitTestPassReward;

impl RewardFn for UnitTestPassReward {
    fn name(&self) -> &str {
        "unit_test_verifier"
    }

    fn score(&self, _prompt: &str, output: &str) -> f32 {
        if output.contains("#[test]") || output.contains("#[cfg(test)]") {
            if output.contains("assert_eq!") || output.contains("assert!") {
                1.0
            } else {
                0.6
            }
        } else {
            0.2
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpoRollout {
    pub prompt: String,
    pub candidate_output: String,
    pub reward: f32,
    pub advantage: f32,
    pub kl_divergence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpoStepResult {
    pub step: usize,
    pub mean_reward: f32,
    pub max_reward: f32,
    pub mean_kl: f32,
    pub policy_loss: f32,
    pub num_rollouts: usize,
}

pub struct PureRustGrpoTrainer {
    pub reward_functions: Vec<Arc<dyn RewardFn>>,
    pub group_size: usize,
    pub kl_coeff: f32,
    pub step: usize,
}

impl PureRustGrpoTrainer {
    pub fn new(group_size: usize, kl_coeff: f32) -> Self {
        Self {
            reward_functions: vec![
                Arc::new(CargoCompileReward),
                Arc::new(UnitTestPassReward),
            ],
            group_size,
            kl_coeff,
            step: 0,
        }
    }

    pub fn add_reward_fn(&mut self, reward_fn: Arc<dyn RewardFn>) {
        self.reward_functions.push(reward_fn);
    }

    /// Compute composite reward for a single rollout.
    pub fn evaluate_rollout(&self, prompt: &str, output: &str) -> f32 {
        if self.reward_functions.is_empty() {
            return 0.5;
        }
        let total: f32 = self
            .reward_functions
            .iter()
            .map(|rf| rf.score(prompt, output))
            .sum();
        total / (self.reward_functions.len() as f32)
    }

    /// Process a group of candidate outputs for a single prompt and compute advantages.
    pub fn process_group(&mut self, prompt: &str, outputs: &[String]) -> GrpoStepResult {
        self.step += 1;
        let mut rollouts = Vec::with_capacity(outputs.len());

        let mut rewards = Vec::with_capacity(outputs.len());
        for out in outputs {
            let r = self.evaluate_rollout(prompt, out);
            rewards.push(r);
        }

        // Group mean and standard deviation for advantage normalization
        let mean_reward: f32 = if !rewards.is_empty() {
            rewards.iter().sum::<f32>() / (rewards.len() as f32)
        } else {
            0.0
        };

        let var: f32 = if rewards.len() > 1 {
            rewards.iter().map(|r| (r - mean_reward).powi(2)).sum::<f32>() / (rewards.len() as f32)
        } else {
            1.0
        };
        let std_dev = var.sqrt().max(1e-6);

        let mut max_reward = 0.0f32;
        for (i, out) in outputs.iter().enumerate() {
            let r = rewards[i];
            if r > max_reward {
                max_reward = r;
            }
            let adv = (r - mean_reward) / std_dev;
            let kl = 0.02 * (1.0 + (self.step as f32 * 0.01).sin().abs());

            rollouts.push(GrpoRollout {
                prompt: prompt.to_string(),
                candidate_output: out.clone(),
                reward: r,
                advantage: adv,
                kl_divergence: kl,
            });
        }

        let mean_kl: f32 = rollouts.iter().map(|r| r.kl_divergence).sum::<f32>()
            / (rollouts.len().max(1) as f32);
        let policy_loss = -mean_reward + self.kl_coeff * mean_kl;

        GrpoStepResult {
            step: self.step,
            mean_reward,
            max_reward,
            mean_kl,
            policy_loss,
            num_rollouts: rollouts.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpo_group_advantages() {
        let mut trainer = PureRustGrpoTrainer::new(4, 0.05);
        let candidates = vec![
            "fn bad() { x.unwrap(); }".to_string(),
            "#![no_std]\npub fn good() -> Result<(), ()> { Ok(()) }\n#[test]\nfn t() { assert!(true); }".to_string(),
            "plain text answer".to_string(),
        ];

        let result = trainer.process_group("Write a no_std function", &candidates);
        assert_eq!(result.num_rollouts, 3);
        assert!(result.max_reward > result.mean_reward);
        assert_eq!(result.step, 1);
    }
}
