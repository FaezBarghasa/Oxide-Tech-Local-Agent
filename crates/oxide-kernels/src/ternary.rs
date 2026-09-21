use oxide_core::OxideError;

/// Fast blockwise Walsh-Hadamard Transform and PTQ1_0 Ternary Dequantization Kernel.
pub struct TernaryHadamardOp {
    pub kernel_name: String,
    pub group_size: usize,
}

impl Default for TernaryHadamardOp {
    fn default() -> Self {
        Self {
            kernel_name: "oxide_ternary_hadamard_ptq1_0".to_string(),
            group_size: 128,
        }
    }
}

impl TernaryHadamardOp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_group_size(group_size: usize) -> Self {
        Self {
            kernel_name: "oxide_ternary_hadamard_ptq1_0".to_string(),
            group_size,
        }
    }

    /// Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars.
    ///
    /// Bit representation:
    ///   00 ->  0.0
    ///   01 ->  1.0
    ///   10 -> -1.0
    ///   11 ->  reserved / 0.0
    pub fn unpack_trits_2bit(&self, packed: &[u8], output: &mut [f32]) -> Result<(), OxideError> {
        let expected_len = packed.len() * 4;
        if output.len() < expected_len {
            return Err(OxideError::Engine(format!(
                "Output buffer too small for unpacked trits: {} < {}",
                output.len(),
                expected_len
            )));
        }

        let mut out_idx = 0;
        for &byte in packed {
            for shift in (0..4).map(|i| i * 2) {
                let code = (byte >> shift) & 0b11;
                output[out_idx] = match code {
                    0b00 => 0.0,
                    0b01 => 1.0,
                    0b10 => -1.0,
                    _ => 0.0,
                };
                out_idx += 1;
            }
        }

        Ok(())
    }

    /// Apply FP16 group scaling factor (g128) to unpacked ternary weights.
    pub fn apply_group_scale(&self, weights: &mut [f32], scales: &[f32]) -> Result<(), OxideError> {
        let groups = weights.len().div_ceil(self.group_size);
        if scales.len() < groups {
            return Err(OxideError::Engine(format!(
                "Scales buffer too small: {} < {}",
                scales.len(),
                groups
            )));
        }

        for (g_idx, chunk) in weights.chunks_mut(self.group_size).enumerate() {
            let scale = scales[g_idx];
            for w in chunk.iter_mut() {
                *w *= scale;
            }
        }

        Ok(())
    }

    /// Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.
    ///
    /// The orthogonal Hadamard rotation spreads activation outliers before ternary quantization,
    /// eliminating the reasoning collapse in 1-bit models like Ternary-Bonsai.
    pub fn fast_hadamard_transform_inplace(&self, block: &mut [f32]) -> Result<(), OxideError> {
        let n = block.len();
        if n == 0 || (n & (n - 1)) != 0 {
            return Err(OxideError::Engine(format!(
                "Hadamard block size must be a non-zero power of 2, got {}",
                n
            )));
        }

        let mut len = 1;
        while len < n {
            for i in (0..n).step_by(len * 2) {
                for j in 0..len {
                    let u = block[i + j];
                    let v = block[i + len + j];
                    block[i + j] = u + v;
                    block[i + len + j] = u - v;
                }
            }
            len *= 2;
        }

        // Normalize by 1 / sqrt(N) for exact orthogonal energy conservation
        let inv_sqrt_n = 1.0 / (n as f32).sqrt();
        for x in block.iter_mut() {
            *x *= inv_sqrt_n;
        }

        Ok(())
    }

    /// Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation.
    pub fn dequantize_and_rotate(
        &self,
        packed_trits: &[u8],
        scales: &[f32],
        output: &mut [f32],
    ) -> Result<(), OxideError> {
        self.unpack_trits_2bit(packed_trits, output)?;
        self.apply_group_scale(output, scales)?;

        // Apply blockwise Hadamard transform per group
        for chunk in output.chunks_mut(self.group_size) {
            self.fast_hadamard_transform_inplace(chunk)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_unpacking() {
        let op = TernaryHadamardOp::new();
        // 0b10_01_00_01: -1.0, 1.0, 0.0, 1.0
        let packed = [0b10_01_00_01];
        let mut out = [0.0f32; 4];
        op.unpack_trits_2bit(&packed, &mut out).unwrap();
        assert_eq!(out, [1.0, 0.0, 1.0, -1.0]);
    }

    #[test]
    fn test_hadamard_orthogonal_energy_conservation() {
        let op = TernaryHadamardOp::new();
        let mut block = [1.0f32, 2.0, 3.0, 4.0];
        let initial_energy: f32 = block.iter().map(|x| x * x).sum();

        op.fast_hadamard_transform_inplace(&mut block).unwrap();
        let transformed_energy: f32 = block.iter().map(|x| x * x).sum();

        // Energy must be preserved within floating point tolerance
        assert!((initial_energy - transformed_energy).abs() < 1e-4);

        // Applying transform twice yields original signal
        op.fast_hadamard_transform_inplace(&mut block).unwrap();
        assert!((block[0] - 1.0).abs() < 1e-4);
        assert!((block[1] - 2.0).abs() < 1e-4);
        assert!((block[2] - 3.0).abs() < 1e-4);
        assert!((block[3] - 4.0).abs() < 1e-4);
    }

    #[test]
    fn test_dequantize_and_rotate_pipeline() {
        let op = TernaryHadamardOp::with_group_size(4);
        let packed = [0b01_01_01_01]; // [1.0, 1.0, 1.0, 1.0]
        let scales = [2.0f32];
        let mut out = [0.0f32; 4];

        op.dequantize_and_rotate(&packed, &scales, &mut out)
            .unwrap();
        // [2.0, 2.0, 2.0, 2.0] through Hadamard H4 is [4.0, 0.0, 0.0, 0.0]
        assert!((out[0] - 4.0).abs() < 1e-4);
        assert!(out[1].abs() < 1e-4);
        assert!(out[2].abs() < 1e-4);
        assert!(out[3].abs() < 1e-4);
    }
}
