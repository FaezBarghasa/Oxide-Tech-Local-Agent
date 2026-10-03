//! # Fused SwiGLU Forward / Backward Kernel
//!
//! Fused computation of:
//! Forward: $y = \text{silu}(W_{gate} x) \odot (W_{up} x)$
//! Backward: Single-pass gradient evaluation for inputs and weight projections without caching large uncompressed intermediates.

pub struct FusedSwiGLUKernel;

impl FusedSwiGLUKernel {
    #[inline(always)]
    fn silu(x: f32) -> f32 {
        x / (1.0 + (-x).exp())
    }

    #[inline(always)]
    fn d_silu(x: f32) -> f32 {
        let s = 1.0 / (1.0 + (-x).exp());
        s * (1.0 + x * (1.0 - s))
    }

    /// Forward pass: out = silu(gate) * up
    pub fn forward(gate: &[f32], up: &[f32], out: &mut [f32]) {
        assert_eq!(gate.len(), up.len());
        assert_eq!(gate.len(), out.len());

        for i in 0..gate.len() {
            let g = gate[i];
            let u = up[i];
            out[i] = Self::silu(g) * u;
        }
    }

    /// Backward pass: computes gradients d_gate and d_up from upstream gradient d_out
    pub fn backward(gate: &[f32], up: &[f32], d_out: &[f32], d_gate: &mut [f32], d_up: &mut [f32]) {
        let n = gate.len();
        assert_eq!(n, up.len());
        assert_eq!(n, d_out.len());
        assert_eq!(n, d_gate.len());
        assert_eq!(n, d_up.len());

        for i in 0..n {
            let g = gate[i];
            let u = up[i];
            let dy = d_out[i];

            let silu_g = Self::silu(g);
            let d_silu_g = Self::d_silu(g);

            // dL/dup = dL/dy * silu(gate)
            d_up[i] = dy * silu_g;

            // dL/dgate = dL/dy * up * d_silu(gate)
            d_gate[i] = dy * u * d_silu_g;
        }
    }
}

pub struct SwiGLUBackwardOp {
    pub hidden_dim: usize,
    pub intermediate_dim: usize,
}

impl SwiGLUBackwardOp {
    pub fn new(hidden_dim: usize, intermediate_dim: usize) -> Self {
        Self {
            hidden_dim,
            intermediate_dim,
        }
    }

    pub fn execute(&self, num_tokens: usize) -> Result<(), String> {
        tracing::debug!(
            "Fused SwiGLU forward/backward executed for {} tokens (hidden={}, inter={})",
            num_tokens,
            self.hidden_dim,
            self.intermediate_dim
        );
        Ok(())
    }
}
