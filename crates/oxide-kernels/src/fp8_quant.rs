//! Block-wise FP8 (E4M3) quantization and dequantization kernels in pure Rust.
//! Designed for low-VRAM LoRA fine-tuning and inference acceleration.

use serde::{Deserialize, Serialize};

/// FP8 E4M3 format: 1 sign bit, 4 exponent bits (bias = 7), 3 mantissa bits.
/// Max value = 448.0, Min positive normal = 2^-6 = 0.015625.
pub const FP8_E4M3_MAX: f32 = 448.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedFP8Block {
    pub data: Vec<u8>,
    pub scales: Vec<f32>,
    pub block_size: usize,
    pub original_len: usize,
}

/// Convert a single f32 to an FP8 E4M3 byte.
#[inline(always)]
pub fn f32_to_fp8_e4m3(val: f32) -> u8 {
    if val == 0.0 {
        return 0;
    }

    let sign = if val.is_sign_negative() { 1u8 << 7 } else { 0u8 };
    let abs_val = val.abs().min(FP8_E4M3_MAX);

    let bits = abs_val.to_bits();
    let exponent = ((bits >> 23) & 0xFF) as i32 - 127;
    let mantissa = bits & 0x7FFFFF;

    if exponent < -6 {
        // Subnormal or underflow
        let shift = (-6 - exponent) as u32;
        if shift > 3 {
            return sign;
        }
        let sub_mantissa = (mantissa | 0x800000) >> (20 + shift);
        return sign | (sub_mantissa as u8 & 0x07);
    }

    let biased_exp = ((exponent + 7).clamp(0, 15)) as u8;
    let rounded_mantissa = ((mantissa >> 20) & 0x07) as u8;

    sign | (biased_exp << 3) | rounded_mantissa
}

/// Convert an FP8 E4M3 byte back to f32.
#[inline(always)]
pub fn fp8_e4m3_to_f32(byte: u8) -> f32 {
    let sign = if (byte & 0x80) != 0 { -1.0f32 } else { 1.0f32 };
    let biased_exp = (byte >> 3) & 0x0F;
    let mantissa = byte & 0x07;

    if biased_exp == 0 {
        if mantissa == 0 {
            return 0.0;
        }
        // Subnormal: (-1)^sign * 2^(-6) * (mantissa / 8)
        sign * (1.0 / 64.0) * (mantissa as f32 / 8.0)
    } else if biased_exp == 15 && mantissa == 7 {
        // NaN / Inf representation in E4M3
        f32::NAN
    } else {
        // Normal: (-1)^sign * 2^(biased_exp - 7) * (1 + mantissa / 8)
        let exp = (biased_exp as i32) - 7;
        let scale = 2.0f32.powi(exp);
        sign * scale * (1.0 + mantissa as f32 / 8.0)
    }
}

/// Quantize a slice of f32 weights into block-wise FP8 with per-block dynamic scaling.
pub fn quantize_fp8_block(tensor: &[f32], block_size: usize) -> QuantizedFP8Block {
    let block_sz = if block_size == 0 { 64 } else { block_size };
    let num_blocks = (tensor.len() + block_sz - 1) / block_sz;

    let mut data = Vec::with_capacity(tensor.len());
    let mut scales = Vec::with_capacity(num_blocks);

    for chunk in tensor.chunks(block_sz) {
        let max_abs = chunk.iter().copied().fold(0.0f32, |m, v| m.max(v.abs()));
        let scale = if max_abs > 1e-8 {
            max_abs / FP8_E4M3_MAX
        } else {
            1.0
        };
        scales.push(scale);

        let inv_scale = 1.0 / scale;
        for &val in chunk {
            let scaled_val = val * inv_scale;
            data.push(f32_to_fp8_e4m3(scaled_val));
        }
    }

    QuantizedFP8Block {
        data,
        scales,
        block_size: block_sz,
        original_len: tensor.len(),
    }
}

/// Dequantize a block-wise FP8 tensor back to f32.
pub fn dequantize_fp8_block(quant: &QuantizedFP8Block) -> Vec<f32> {
    let mut result = Vec::with_capacity(quant.original_len);

    for (b_idx, chunk) in quant.data.chunks(quant.block_size).enumerate() {
        let scale = quant.scales.get(b_idx).copied().unwrap_or(1.0);
        for &byte in chunk {
            let base_val = fp8_e4m3_to_f32(byte);
            result.push(base_val * scale);
        }
    }

    result.truncate(quant.original_len);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp8_roundtrip() {
        let input = vec![0.0, 1.0, -1.0, 0.5, -0.25, 42.0, -128.0, 350.0];
        let quant = quantize_fp8_block(&input, 4);
        assert_eq!(quant.scales.len(), 2);

        let recovered = dequantize_fp8_block(&quant);
        assert_eq!(recovered.len(), input.len());

        for (orig, rec) in input.iter().zip(recovered.iter()) {
            let rel_err = if orig.abs() > 1e-5 {
                (orig - rec).abs() / orig.abs()
            } else {
                (orig - rec).abs()
            };
            assert!(rel_err < 0.25, "Value {} vs recovered {}", orig, rec);
        }
    }
}
