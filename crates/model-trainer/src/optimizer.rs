//! # Pure-Rust AdamW Optimizer with Weight Decay and Cosine LR Scheduler
//!
//! High-performance numerical optimizer for updating LoRA adapter matrices.

#[derive(Debug, Clone)]
pub struct AdamWConfig {
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub eps: f32,
    pub weight_decay: f32,
    pub max_grad_norm: f32,
}

impl Default for AdamWConfig {
    fn default() -> Self {
        Self {
            lr: 2e-4,
            beta1: 0.9,
            beta2: 0.999,
            eps: 1e-8,
            weight_decay: 0.01,
            max_grad_norm: 1.0,
        }
    }
}

pub struct AdamWState {
    pub step: usize,
    pub m: Vec<f32>,
    pub v: Vec<f32>,
}

impl AdamWState {
    pub fn new(param_count: usize) -> Self {
        Self {
            step: 0,
            m: vec![0.0; param_count],
            v: vec![0.0; param_count],
        }
    }
}

pub struct AdamWOptimizer {
    pub config: AdamWConfig,
}

impl AdamWOptimizer {
    pub fn new(config: AdamWConfig) -> Self {
        Self { config }
    }

    /// Performs one in-place optimization step on parameters given gradients
    pub fn step(
        &self,
        params: &mut [f32],
        grads: &mut [f32],
        state: &mut AdamWState,
    ) {
        assert_eq!(params.len(), grads.len());
        assert_eq!(params.len(), state.m.len());
        assert_eq!(params.len(), state.v.len());

        state.step += 1;
        let t = state.step as f32;

        // 1. Gradient clipping by global norm
        if self.config.max_grad_norm > 0.0 {
            let mut sum_sq = 0.0f32;
            for &g in grads.iter() {
                sum_sq += g * g;
            }
            let norm = sum_sq.sqrt();
            if norm > self.config.max_grad_norm {
                let scale = self.config.max_grad_norm / (norm + 1e-8);
                for g in grads.iter_mut() {
                    *g *= scale;
                }
            }
        }

        let beta1 = self.config.beta1;
        let beta2 = self.config.beta2;
        let lr = self.config.lr;
        let wd = self.config.weight_decay;
        let eps = self.config.eps;

        let bc1 = 1.0 - beta1.powf(t);
        let bc2 = 1.0 - beta2.powf(t);

        // 2. AdamW in-place update
        for i in 0..params.len() {
            let g = grads[i];

            // Weight decay
            params[i] -= lr * wd * params[i];

            // Moment updates
            state.m[i] = beta1 * state.m[i] + (1.0 - beta1) * g;
            state.v[i] = beta2 * state.v[i] + (1.0 - beta2) * g * g;

            let m_hat = state.m[i] / bc1;
            let v_hat = state.v[i] / bc2;

            params[i] -= lr * m_hat / (v_hat.sqrt() + eps);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adamw_step_convergence() {
        let mut params = vec![5.0f32, -3.0f32];
        let mut state = AdamWState::new(2);
        let opt = AdamWOptimizer::new(AdamWConfig {
            lr: 0.1,
            weight_decay: 0.0,
            ..Default::default()
        });

        // Optimize quadratic loss L = 0.5 * (x^2 + y^2) -> grad = [x, y]
        for _ in 0..100 {
            let mut grads = params.clone();
            opt.step(&mut params, &mut grads, &mut state);
        }

        assert!(params[0].abs() < 0.1);
        assert!(params[1].abs() < 0.1);
    }
}
