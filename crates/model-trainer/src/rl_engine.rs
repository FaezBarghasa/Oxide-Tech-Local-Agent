use serde::{Deserialize, Serialize};

/// Preference Pair (Chosen vs. Rejected completions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceSample {
    pub prompt: String,
    pub chosen: String,
    pub rejected: String,
}

/// DPO (Direct Preference Optimization) Loss
pub struct DpoLoss {
    pub beta: f32,
    pub label_smoothing: f32,
}

impl Default for DpoLoss {
    fn default() -> Self {
        Self {
            beta: 0.1,
            label_smoothing: 0.0,
        }
    }
}

impl DpoLoss {
    pub fn new(beta: f32) -> Self {
        Self {
            beta,
            label_smoothing: 0.0,
        }
    }

    /// Compute DPO loss from policy and reference log probabilities
    /// Loss = -log(sigmoid(beta * (log(pi_theta(yw)/ref(yw)) - log(pi_theta(yl)/ref(yl)))))
    pub fn forward(
        &self,
        pi_chosen_logp: f32,
        pi_rejected_logp: f32,
        ref_chosen_logp: f32,
        ref_rejected_logp: f32,
    ) -> (f32, f32, f32) {
        let chosen_logratio = pi_chosen_logp - ref_chosen_logp;
        let rejected_logratio = pi_rejected_logp - ref_rejected_logp;
        let logits = self.beta * (chosen_logratio - rejected_logratio);

        // Sigmoid log loss
        let loss = if logits > 0.0 {
            (1.0 + (-logits).exp()).ln()
        } else {
            -logits + (1.0 + logits.exp()).ln()
        };

        let implicit_reward_chosen = self.beta * chosen_logratio;
        let implicit_reward_rejected = self.beta * rejected_logratio;

        (loss, implicit_reward_chosen, implicit_reward_rejected)
    }
}

/// ORPO (Odds Ratio Preference Optimization) Loss
pub struct OrpoLoss {
    pub lambda_odds: f32,
}

impl Default for OrpoLoss {
    fn default() -> Self {
        Self { lambda_odds: 0.1 }
    }
}

impl OrpoLoss {
    pub fn new(lambda_odds: f32) -> Self {
        Self { lambda_odds }
    }

    /// Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty
    pub fn forward(&self, sft_nll_loss: f32, pi_chosen_logp: f32, pi_rejected_logp: f32) -> f32 {
        // Odds = p / (1 - p) -> log(Odds) ~ logp - log(1 - exp(logp))
        // Simplified stable log odds ratio
        let log_odds_chosen = pi_chosen_logp - (1.0 - pi_chosen_logp.exp().min(0.9999)).ln();
        let log_odds_rejected = pi_rejected_logp - (1.0 - pi_rejected_logp.exp().min(0.9999)).ln();
        let odds_diff = log_odds_chosen - log_odds_rejected;

        let odds_ratio_loss = (1.0 + (-odds_diff).exp()).ln();
        sft_nll_loss + self.lambda_odds * odds_ratio_loss
    }
}

/// GRPO (Group Relative Policy Optimization) Engine for Reasoning RL
pub struct GrpoEngine {
    pub group_size: usize,
    pub clip_eps: f32,
    pub beta_kl: f32,
}

impl Default for GrpoEngine {
    fn default() -> Self {
        Self {
            group_size: 8,
            clip_eps: 0.2,
            beta_kl: 0.04,
        }
    }
}

impl GrpoEngine {
    pub fn new(group_size: usize, clip_eps: f32, beta_kl: f32) -> Self {
        Self {
            group_size,
            clip_eps,
            beta_kl,
        }
    }

    /// Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)
    pub fn compute_group_advantages(&self, rewards: &[f32]) -> Vec<f32> {
        if rewards.is_empty() {
            return Vec::new();
        }

        let n = rewards.len() as f32;
        let mean = rewards.iter().sum::<f32>() / n;
        let variance = rewards.iter().map(|r| (r - mean).powi(2)).sum::<f32>() / (n + 1e-8);
        let std = (variance + 1e-8).sqrt();

        rewards.iter().map(|r| (r - mean) / std).collect()
    }

    /// Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty
    pub fn compute_policy_loss(
        &self,
        pi_logp: &[f32],
        old_logp: &[f32],
        ref_logp: &[f32],
        advantage: f32,
    ) -> f32 {
        let mut total_loss = 0.0;
        let count = pi_logp.len().min(old_logp.len()).min(ref_logp.len());

        if count == 0 {
            return 0.0;
        }

        for i in 0..count {
            let ratio = (pi_logp[i] - old_logp[i]).exp();
            let surr1 = ratio * advantage;
            let surr2 = ratio.clamp(1.0 - self.clip_eps, 1.0 + self.clip_eps) * advantage;
            let ppo_loss = -surr1.min(surr2);

            // KL penalty: D_KL(pi || ref) ~ exp(ref - pi) - (ref - pi) - 1 (or pi_logp - ref_logp)
            let kl = (pi_logp[i] - ref_logp[i]).exp() - (pi_logp[i] - ref_logp[i]) - 1.0;
            total_loss += ppo_loss + self.beta_kl * kl;
        }

        total_loss / count as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpo_loss_computation() {
        let dpo = DpoLoss::new(0.1);
        let (loss, rew_w, rew_l) = dpo.forward(-1.2, -3.5, -1.5, -3.0);
        assert!(loss > 0.0);
        assert!(rew_w > rew_l);
    }

    #[test]
    fn test_grpo_group_advantage_normalization() {
        let grpo = GrpoEngine::default();
        let rewards = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let advantages = grpo.compute_group_advantages(&rewards);

        assert_eq!(advantages.len(), 5);
        let sum: f32 = advantages.iter().sum();
        assert!(sum.abs() < 1e-4); // Mean centered at 0
        assert!(advantages[4] > advantages[0]);
    }
}
