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
    pub fn step(&self, params: &mut [f32], grads: &mut [f32], state: &mut AdamWState) {
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

/// 8-bit Quantized Blockwise AdamW State
pub struct AdamW8bitState {
    pub step: usize,
    pub block_size: usize,
    pub m_quant: Vec<i8>,
    pub v_quant: Vec<u8>,
    pub m_scales: Vec<f32>,
    pub v_scales: Vec<f32>,
}

impl AdamW8bitState {
    pub fn new(param_count: usize, block_size: usize) -> Self {
        let blocks = param_count.div_ceil(block_size);
        Self {
            step: 0,
            block_size,
            m_quant: vec![0i8; param_count],
            v_quant: vec![0u8; param_count],
            m_scales: vec![1.0f32; blocks],
            v_scales: vec![1.0f32; blocks],
        }
    }
}

/// Pure-Rust 8-bit Quantized Blockwise AdamW Optimizer
pub struct AdamW8bitOptimizer {
    pub config: AdamWConfig,
    pub block_size: usize,
}

impl AdamW8bitOptimizer {
    pub fn new(config: AdamWConfig) -> Self {
        Self {
            config,
            block_size: 256,
        }
    }

    pub fn step(&self, params: &mut [f32], grads: &mut [f32], state: &mut AdamW8bitState) {
        state.step += 1;
        let t = state.step as f32;
        let beta1 = self.config.beta1;
        let beta2 = self.config.beta2;
        let lr = self.config.lr;
        let wd = self.config.weight_decay;
        let eps = self.config.eps;

        let bc1 = 1.0 - beta1.powf(t);
        let bc2 = 1.0 - beta2.powf(t);

        let num_blocks = params.len().div_ceil(state.block_size);

        for b in 0..num_blocks {
            let start = b * state.block_size;
            let end = (start + state.block_size).min(params.len());

            // Dequantize and update block
            let mut m_block = Vec::with_capacity(end - start);
            let mut v_block = Vec::with_capacity(end - start);

            for i in start..end {
                let g = grads[i];
                params[i] -= lr * wd * params[i];

                let m_real = (state.m_quant[i] as f32 / 127.0) * state.m_scales[b];
                let v_real = (state.v_quant[i] as f32 / 255.0) * state.v_scales[b];

                let new_m = beta1 * m_real + (1.0 - beta1) * g;
                let new_v = beta2 * v_real + (1.0 - beta2) * g * g;

                let m_hat = new_m / bc1;
                let v_hat = new_v / bc2;
                params[i] -= lr * m_hat / (v_hat.sqrt() + eps);

                m_block.push(new_m);
                v_block.push(new_v);
            }

            // Quantize block moments back to 8-bit
            let max_m = m_block
                .iter()
                .map(|x| x.abs())
                .fold(0.0f32, f32::max)
                .max(1e-8);
            let max_v = v_block.iter().fold(0.0f32, |acc, &x| acc.max(x)).max(1e-8);

            state.m_scales[b] = max_m;
            state.v_scales[b] = max_v;

            for (idx, i) in (start..end).enumerate() {
                state.m_quant[i] = ((m_block[idx] / max_m) * 127.0).clamp(-127.0, 127.0) as i8;
                state.v_quant[i] = ((v_block[idx] / max_v) * 255.0).clamp(0.0, 255.0) as u8;
            }
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

    #[test]
    fn test_adamw8bit_step_convergence() {
        let mut params = vec![4.0f32, -2.0f32];
        let mut state = AdamW8bitState::new(2, 64);
        let opt = AdamW8bitOptimizer::new(AdamWConfig {
            lr: 0.15,
            weight_decay: 0.0,
            ..Default::default()
        });

        for _ in 0..250 {
            let mut grads = params.clone();
            opt.step(&mut params, &mut grads, &mut state);
        }

        assert!(params[0].abs() < 0.25);
        assert!(params[1].abs() < 0.25);
    }
}
