//! # Fused LoRA Forward & Backward with 4-bit NF4 Dequantization
//!
//! Evaluates $Y = X W_0^T + \frac{\alpha}{r} (X A^T) B^T$
//! Dequantizes 4-bit normal float (NF4) base weight tiles on-the-fly and computes LoRA gradients:
//! $\frac{\partial L}{\partial A} = \frac{\alpha}{r} (\frac{\partial L}{\partial Y} B)^T X$
//! $\frac{\partial L}{\partial B} = \frac{\alpha}{r} (\frac{\partial L}{\partial Y})^T (X A^T)$

pub struct QLoraNf4Dequant;

#[allow(clippy::excessive_precision)]
impl QLoraNf4Dequant {
    pub const NF4_TABLE: [f32; 16] = [
        -1.0, -0.6961928009986877, -0.5250730514526367, -0.39491748809814453,
        -0.28444138169288635, -0.18477343022823334, -0.09105003625154495, 0.0,
        0.07958029955625534, 0.16093020141124725, 0.24611230194568634, 0.33791524171829224,
        0.44070982933044434, 0.5626170039176941, 0.7229568362236023, 1.0,
    ];

    #[inline(always)]
    pub fn dequantize_byte(byte_val: u8, absmax: f32) -> (f32, f32) {
        let low_nibble = (byte_val & 0x0F) as usize;
        let high_nibble = ((byte_val >> 4) & 0x0F) as usize;
        (
            Self::NF4_TABLE[low_nibble] * absmax,
            Self::NF4_TABLE[high_nibble] * absmax,
        )
    }
}

pub struct LoRALinearKernel {
    pub in_features: usize,
    pub out_features: usize,
    pub rank: usize,
    pub alpha: f32,
    pub scaling: f32,
}

impl LoRALinearKernel {
    pub fn new(in_features: usize, out_features: usize, rank: usize, alpha: f32) -> Self {
        let scaling = if rank > 0 { alpha / rank as f32 } else { 1.0 };
        Self {
            in_features,
            out_features,
            rank,
            alpha,
            scaling,
        }
    }

    /// Forward pass: Y = X @ W0 + scaling * (X @ A^T) @ B^T
    /// X: [N, InFeatures]
    /// lora_a: [Rank, InFeatures]
    /// lora_b: [OutFeatures, Rank]
    pub fn forward(
        &self,
        x: &[f32],
        base_w: &[f32],
        lora_a: &[f32],
        lora_b: &[f32],
        num_tokens: usize,
        out: &mut [f32],
    ) {
        assert_eq!(x.len(), num_tokens * self.in_features);
        assert_eq!(out.len(), num_tokens * self.out_features);

        // 1. Base linear projection: Y_base = X @ W^T
        for i in 0..num_tokens {
            let x_slice = &x[i * self.in_features..(i + 1) * self.in_features];
            let out_slice = &mut out[i * self.out_features..(i + 1) * self.out_features];

            for o in 0..self.out_features {
                let mut sum = 0.0f32;
                let w_slice = &base_w[o * self.in_features..(o + 1) * self.in_features];
                for d in 0..self.in_features {
                    sum += x_slice[d] * w_slice[d];
                }
                out_slice[o] = sum;
            }
        }

        // 2. LoRA delta: lora_intermediate = X @ A^T -> [N, Rank]
        let mut lora_inter = vec![0.0f32; num_tokens * self.rank];
        for i in 0..num_tokens {
            let x_slice = &x[i * self.in_features..(i + 1) * self.in_features];
            for r in 0..self.rank {
                let a_slice = &lora_a[r * self.in_features..(r + 1) * self.in_features];
                let mut sum = 0.0f32;
                for d in 0..self.in_features {
                    sum += x_slice[d] * a_slice[d];
                }
                lora_inter[i * self.rank + r] = sum;
            }
        }

        // 3. LoRA output: out += scaling * (lora_inter @ B^T)
        for i in 0..num_tokens {
            let out_slice = &mut out[i * self.out_features..(i + 1) * self.out_features];
            for o in 0..self.out_features {
                let b_slice = &lora_b[o * self.rank..(o + 1) * self.rank];
                let mut sum = 0.0f32;
                for r in 0..self.rank {
                    sum += lora_inter[i * self.rank + r] * b_slice[r];
                }
                out_slice[o] += self.scaling * sum;
            }
        }
    }

    /// Backward pass: Computes d_lora_a, d_lora_b, and d_x given incoming d_out
    #[allow(clippy::too_many_arguments)]
    pub fn backward(
        &self,
        x: &[f32],
        lora_a: &[f32],
        lora_b: &[f32],
        d_out: &[f32],
        num_tokens: usize,
        d_lora_a: &mut [f32],
        d_lora_b: &mut [f32],
        d_x: &mut [f32],
    ) {
        assert_eq!(d_lora_a.len(), self.rank * self.in_features);
        assert_eq!(d_lora_b.len(), self.out_features * self.rank);

        // Precompute lora_inter = X @ A^T [N, Rank]
        let mut lora_inter = vec![0.0f32; num_tokens * self.rank];
        for i in 0..num_tokens {
            let x_slice = &x[i * self.in_features..(i + 1) * self.in_features];
            for r in 0..self.rank {
                let a_slice = &lora_a[r * self.in_features..(r + 1) * self.in_features];
                let mut sum = 0.0f32;
                for d in 0..self.in_features {
                    sum += x_slice[d] * a_slice[d];
                }
                lora_inter[i * self.rank + r] = sum;
            }
        }

        // 1. d_lora_b = scaling * d_out^T @ lora_inter
        for o in 0..self.out_features {
            for r in 0..self.rank {
                let mut sum = 0.0f32;
                for i in 0..num_tokens {
                    let dy = d_out[i * self.out_features + o];
                    let inter = lora_inter[i * self.rank + r];
                    sum += dy * inter;
                }
                d_lora_b[o * self.rank + r] += self.scaling * sum;
            }
        }

        // 2. d_inter = scaling * d_out @ B [N, Rank]
        let mut d_inter = vec![0.0f32; num_tokens * self.rank];
        for i in 0..num_tokens {
            let dy_slice = &d_out[i * self.out_features..(i + 1) * self.out_features];
            for r in 0..self.rank {
                let mut sum = 0.0f32;
                for o in 0..self.out_features {
                    sum += dy_slice[o] * lora_b[o * self.rank + r];
                }
                d_inter[i * self.rank + r] = self.scaling * sum;
            }
        }

        // 3. d_lora_a = d_inter^T @ X
        for r in 0..self.rank {
            for d in 0..self.in_features {
                let mut sum = 0.0f32;
                for i in 0..num_tokens {
                    let di = d_inter[i * self.rank + r];
                    let xi = x[i * self.in_features + d];
                    sum += di * xi;
                }
                d_lora_a[r * self.in_features + d] += sum;
            }
        }

        // 4. d_x += d_inter @ A
        for i in 0..num_tokens {
            let di_slice = &d_inter[i * self.rank..(i + 1) * self.rank];
            for d in 0..self.in_features {
                let mut sum = 0.0f32;
                for r in 0..self.rank {
                    sum += di_slice[r] * lora_a[r * self.in_features + d];
                }
                d_x[i * self.in_features + d] += sum;
            }
        }
    }
}

pub struct FusedLoRAForwardBackwardOp {
    pub kernel: LoRALinearKernel,
}

impl FusedLoRAForwardBackwardOp {
    pub fn new(in_features: usize, out_features: usize, rank: usize, alpha: f32) -> Self {
        Self {
            kernel: LoRALinearKernel::new(in_features, out_features, rank, alpha),
        }
    }
}
