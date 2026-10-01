//! FP8 LoRA Layer for pure-Rust memory-efficient adapter fine-tuning.

use crate::fp8_quant::{dequantize_fp8_block, quantize_fp8_block, QuantizedFP8Block};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FP8Matrix {
    pub rows: usize,
    pub cols: usize,
    pub quant: QuantizedFP8Block,
}

impl FP8Matrix {
    pub fn from_f32_slice(slice: &[f32], rows: usize, cols: usize, block_size: usize) -> Self {
        assert_eq!(slice.len(), rows * cols, "Slice length must equal rows * cols");
        let quant = quantize_fp8_block(slice, block_size);
        Self { rows, cols, quant }
    }

    pub fn to_f32_vec(&self) -> Vec<f32> {
        dequantize_fp8_block(&self.quant)
    }
}

/// FP8 Low-Rank Adapter layer: W_eff = W_base + (alpha / rank) * (A @ B)
/// Where A is (in_features x rank) and B is (rank x out_features)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FP8LoraLayer {
    pub in_features: usize,
    pub out_features: usize,
    pub rank: usize,
    pub alpha: f32,
    pub a_matrix: FP8Matrix,
    pub b_matrix: FP8Matrix,
    pub scaling: f32,
}

impl FP8LoraLayer {
    pub fn new(in_features: usize, out_features: usize, rank: usize, alpha: f32) -> Self {
        let scaling = alpha / (rank as f32);
        let block_sz = 64;

        // Initialize A with normal distribution / Kaiming uniform (mock initialization in f32)
        let mut a_raw = vec![0.0f32; in_features * rank];
        let std_dev = (2.0 / (in_features as f32)).sqrt();
        for (i, v) in a_raw.iter_mut().enumerate() {
            *v = (((i * 1337 + 7) % 1000) as f32 / 1000.0 - 0.5) * std_dev;
        }

        // Initialize B with zeros so initial delta is 0
        let b_raw = vec![0.0f32; rank * out_features];

        let a_matrix = FP8Matrix::from_f32_slice(&a_raw, in_features, rank, block_sz);
        let b_matrix = FP8Matrix::from_f32_slice(&b_raw, rank, out_features, block_sz);

        Self {
            in_features,
            out_features,
            rank,
            alpha,
            a_matrix,
            b_matrix,
            scaling,
        }
    }

    /// Forward pass: output = input @ A @ B * scaling
    /// input: (batch_size, in_features)
    /// returns: (batch_size, out_features)
    pub fn forward(&self, input: &[f32], batch_size: usize) -> Vec<f32> {
        assert_eq!(input.len(), batch_size * self.in_features);

        let a_f32 = self.a_matrix.to_f32_vec();
        let b_f32 = self.b_matrix.to_f32_vec();

        // 1. intermediate = input @ A -> (batch_size x rank)
        let mut intermediate = vec![0.0f32; batch_size * self.rank];
        for b in 0..batch_size {
            for r in 0..self.rank {
                let mut sum = 0.0f32;
                for i in 0..self.in_features {
                    let in_val = input[b * self.in_features + i];
                    let a_val = a_f32[i * self.rank + r];
                    sum += in_val * a_val;
                }
                intermediate[b * self.rank + r] = sum;
            }
        }

        // 2. output = intermediate @ B * scaling -> (batch_size x out_features)
        let mut output = vec![0.0f32; batch_size * self.out_features];
        for b in 0..batch_size {
            for o in 0..self.out_features {
                let mut sum = 0.0f32;
                for r in 0..self.rank {
                    let inter_val = intermediate[b * self.rank + r];
                    let b_val = b_f32[r * self.out_features + o];
                    sum += inter_val * b_val;
                }
                output[b * self.out_features + o] = sum * self.scaling;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp8_lora_layer_forward() {
        let layer = FP8LoraLayer::new(16, 8, 4, 8.0);
        let input = vec![1.0f32; 16]; // batch_size = 1
        let out = layer.forward(&input, 1);
        assert_eq!(out.len(), 8);
        // Since B starts as zeros, initial forward should output all zeros
        for val in out {
            assert_eq!(val, 0.0);
        }
    }
}
