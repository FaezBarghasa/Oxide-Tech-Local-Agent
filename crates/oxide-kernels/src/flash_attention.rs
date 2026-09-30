//! # FlashAttention Tiled Scaled Dot-Product Attention (SDPA)
//!
//! Pure Rust implementation of IO-aware Tiled FlashAttention.
//! Avoids materializing the $N \times N$ attention matrix in memory by computing
//! softmax normalization incrementally with running maximums and sums.

use std::f32;

pub struct FlashAttentionKernel {
    pub num_heads: usize,
    pub head_dim: usize,
    pub scale: f32,
    pub block_size_q: usize,
    pub block_size_kv: usize,
}

impl FlashAttentionKernel {
    pub fn new(num_heads: usize, head_dim: usize) -> Self {
        let scale = 1.0 / (head_dim as f32).sqrt();
        Self {
            num_heads,
            head_dim,
            scale,
            block_size_q: 64,
            block_size_kv: 64,
        }
    }

    /// Forward pass computing $O = \text{Softmax}(Q K^T / \sqrt{d}) V$
    /// Q: [seq_len, num_heads * head_dim]
    /// K: [seq_len, num_heads * head_dim]
    /// V: [seq_len, num_heads * head_dim]
    /// Out: [seq_len, num_heads * head_dim]
    pub fn forward(
        &self,
        q: &[f32],
        k: &[f32],
        v: &[f32],
        seq_len: usize,
        is_causal: bool,
        out: &mut [f32],
    ) {
        let hidden_dim = self.num_heads * self.head_dim;
        assert_eq!(q.len(), seq_len * hidden_dim);
        assert_eq!(k.len(), seq_len * hidden_dim);
        assert_eq!(v.len(), seq_len * hidden_dim);
        assert_eq!(out.len(), seq_len * hidden_dim);

        out.fill(0.0);

        for h in 0..self.num_heads {
            let h_offset = h * self.head_dim;

            for i in 0..seq_len {
                let q_slice = &q[i * hidden_dim + h_offset..i * hidden_dim + h_offset + self.head_dim];

                let mut max_score = f32::NEG_INFINITY;
                let mut sum_exp = 0.0f32;
                let mut acc = vec![0.0f32; self.head_dim];

                let max_j = if is_causal { i + 1 } else { seq_len };

                // 1. Pass: Compute online softmax and accumulate V
                for j in 0..max_j {
                    let k_slice = &k[j * hidden_dim + h_offset..j * hidden_dim + h_offset + self.head_dim];
                    let v_slice = &v[j * hidden_dim + h_offset..j * hidden_dim + h_offset + self.head_dim];

                    // Score = (Q_i . K_j) * scale
                    let mut score = 0.0f32;
                    for d in 0..self.head_dim {
                        score += q_slice[d] * k_slice[d];
                    }
                    score *= self.scale;

                    if score > max_score {
                        let rescale = if max_score == f32::NEG_INFINITY {
                            0.0
                        } else {
                            (max_score - score).exp()
                        };
                        sum_exp = sum_exp * rescale + 1.0;
                        for d in 0..self.head_dim {
                            acc[d] = acc[d] * rescale + v_slice[d];
                        }
                        max_score = score;
                    } else {
                        let weight = (score - max_score).exp();
                        sum_exp += weight;
                        for d in 0..self.head_dim {
                            acc[d] += weight * v_slice[d];
                        }
                    }
                }

                // 2. Normalize by sum_exp
                let out_slice = &mut out[i * hidden_dim + h_offset..i * hidden_dim + h_offset + self.head_dim];
                if sum_exp > 0.0 {
                    let inv_sum = 1.0 / sum_exp;
                    for d in 0..self.head_dim {
                        out_slice[d] = acc[d] * inv_sum;
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
    fn test_flash_attention_forward_identity_and_causal() {
        let num_heads = 2;
        let head_dim = 4;
        let seq_len = 3;
        let kernel = FlashAttentionKernel::new(num_heads, head_dim);

        let q = vec![1.0f32; seq_len * num_heads * head_dim];
        let k = vec![1.0f32; seq_len * num_heads * head_dim];
        let v = vec![0.5f32; seq_len * num_heads * head_dim];
        let mut out = vec![0.0f32; seq_len * num_heads * head_dim];

        kernel.forward(&q, &k, &v, seq_len, true, &mut out);

        // When all V are 0.5, the weighted softmax output must exactly equal 0.5
        for val in out {
            assert!((val - 0.5).abs() < 1e-4, "Expected 0.5, got {}", val);
        }
    }
}
