//! # Pure-Rust LoRA SFT Training Engine
//!
//! End-to-end execution loop:
//! 1. Forward pass via `LoRALinearKernel` & `ChunkedCrossEntropyKernel`
//! 2. Analytical gradient backward pass computing $\frac{\partial L}{\partial A}$ and $\frac{\partial L}{\partial B}$
//! 3. In-place AdamW parameter update with weight decay

use oxide_kernels::{ChunkedCrossEntropyConfig, ChunkedCrossEntropyKernel, LoRALinearKernel};
use crate::optimizer::{AdamWConfig, AdamWOptimizer, AdamWState};

pub struct LoRATrainingEngine {
    pub in_features: usize,
    pub out_features: usize,
    pub rank: usize,
    pub alpha: f32,
    pub lora_a: Vec<f32>,
    pub lora_b: Vec<f32>,
    pub optimizer_a: AdamWOptimizer,
    pub optimizer_b: AdamWOptimizer,
    pub state_a: AdamWState,
    pub state_b: AdamWState,
    pub kernel: LoRALinearKernel,
}

impl LoRATrainingEngine {
    pub fn new(
        in_features: usize,
        out_features: usize,
        rank: usize,
        alpha: f32,
        opt_cfg: AdamWConfig,
    ) -> Self {
        let kernel = LoRALinearKernel::new(in_features, out_features, rank, alpha);
        
        // LoRA A: standard small initialization (e.g. 0.01)
        let lora_a = vec![0.01f32; rank * in_features];
        // LoRA B: zero initialization (ensures delta is initially 0)
        let lora_b = vec![0.0f32; out_features * rank];

        let state_a = AdamWState::new(rank * in_features);
        let state_b = AdamWState::new(out_features * rank);

        Self {
            in_features,
            out_features,
            rank,
            alpha,
            lora_a,
            lora_b,
            optimizer_a: AdamWOptimizer::new(opt_cfg.clone()),
            optimizer_b: AdamWOptimizer::new(opt_cfg),
            state_a,
            state_b,
            kernel,
        }
    }

    /// Executes one training step over a token batch
    pub fn train_step(
        &mut self,
        x: &[f32],
        base_weight: &[f32],
        targets: &[i64],
        num_tokens: usize,
    ) -> Result<f32, String> {
        let mut logits = vec![0.0f32; num_tokens * self.out_features];
        
        // 1. Forward pass
        self.kernel.forward(
            x,
            base_weight,
            &self.lora_a,
            &self.lora_b,
            num_tokens,
            &mut logits,
        );

        // 2. Fused Cross-Entropy Loss computation
        let ce_cfg = ChunkedCrossEntropyConfig {
            chunk_size: 64,
            label_smoothing: 0.0,
            ignore_index: -100,
        };
        let ce_kernel = ChunkedCrossEntropyKernel::new(ce_cfg);
        // Identity matrix for LM head to compute loss directly over logits
        let mut eye_head = vec![0.0f32; self.out_features * self.out_features];
        for i in 0..self.out_features {
            eye_head[i * self.out_features + i] = 1.0;
        }

        let (loss, _) = ce_kernel.compute_loss(
            &logits,
            &eye_head,
            targets,
            self.out_features,
            self.out_features,
        )?;

        // 3. Compute gradient of cross-entropy w.r.t logits: d_logits = (Softmax(logits) - 1_y) / N
        let mut d_logits = vec![0.0f32; num_tokens * self.out_features];
        for i in 0..num_tokens {
            let logit_slice = &logits[i * self.out_features..(i + 1) * self.out_features];
            let target_token = targets[i];

            let max_val = logit_slice.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let mut sum_exp = 0.0f32;
            for &val in logit_slice {
                sum_exp += (val - max_val).exp();
            }

            let d_slice = &mut d_logits[i * self.out_features..(i + 1) * self.out_features];
            let inv_n = 1.0 / num_tokens as f32;
            for v in 0..self.out_features {
                let prob = (logit_slice[v] - max_val).exp() / (sum_exp + 1e-12);
                let target_indicator = if (v as i64) == target_token { 1.0 } else { 0.0 };
                d_slice[v] = (prob - target_indicator) * inv_n;
            }
        }

        // 4. LoRA analytical backward pass
        let mut d_lora_a = vec![0.0f32; self.rank * self.in_features];
        let mut d_lora_b = vec![0.0f32; self.out_features * self.rank];
        let mut d_x = vec![0.0f32; num_tokens * self.in_features];

        self.kernel.backward(
            x,
            &self.lora_a,
            &self.lora_b,
            &d_logits,
            num_tokens,
            &mut d_lora_a,
            &mut d_lora_b,
            &mut d_x,
        );

        // 5. AdamW optimizer update
        self.optimizer_a.step(&mut self.lora_a, &mut d_lora_a, &mut self.state_a);
        self.optimizer_b.step(&mut self.lora_b, &mut d_lora_b, &mut self.state_b);

        Ok(loss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lora_training_step_reduces_loss() {
        let in_features = 8;
        let out_features = 4;
        let rank = 2;
        let alpha = 4.0;
        let num_tokens = 2;

        let mut engine = LoRATrainingEngine::new(
            in_features,
            out_features,
            rank,
            alpha,
            AdamWConfig {
                lr: 0.05,
                weight_decay: 0.0,
                ..Default::default()
            },
        );

        let x = vec![1.0f32; num_tokens * in_features];
        let base_w = vec![0.1f32; out_features * in_features];
        let targets = vec![1i64, 3i64];

        let initial_loss = engine.train_step(&x, &base_w, &targets, num_tokens).unwrap();

        for _ in 0..20 {
            let _ = engine.train_step(&x, &base_w, &targets, num_tokens).unwrap();
        }

        let final_loss = engine.train_step(&x, &base_w, &targets, num_tokens).unwrap();
        assert!(final_loss < initial_loss, "Expected final loss {} < initial loss {}", final_loss, initial_loss);
    }
}
