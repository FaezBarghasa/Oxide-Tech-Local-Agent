use oxide_core::OxideError;

/// Fused Root-Mean-Square Normalization (RMSNorm) operator with optional fused residual addition.
///
/// Combines residual accumulation, variance reduction, and weight scaling into
/// a single memory pass, eliminating redundant DRAM reads and writes.
pub struct FusedRmsNormOp {
    pub kernel_name: String,
    pub eps: f32,
}

impl Default for FusedRmsNormOp {
    fn default() -> Self {
        Self {
            kernel_name: "oxide_fused_rmsnorm".to_string(),
            eps: 1e-6,
        }
    }
}

impl FusedRmsNormOp {
    pub fn new(eps: f32) -> Self {
        Self {
            kernel_name: "oxide_fused_rmsnorm".to_string(),
            eps,
        }
    }

    /// Apply in-place RMSNorm:
    ///   x = (x / sqrt(mean(x^2) + eps)) * weight
    pub fn forward(&self, x: &mut [f32], weight: &[f32]) -> Result<(), OxideError> {
        let dim = x.len();
        if dim == 0 || weight.len() != dim {
            return Err(OxideError::Engine(format!(
                "Dimension mismatch: input len {}, weight len {}",
                dim,
                weight.len()
            )));
        }

        // Calculate mean of squares
        let sum_sq: f32 = x.iter().map(|&v| v * v).sum();
        let rms = (sum_sq / dim as f32 + self.eps).sqrt();
        let inv_rms = 1.0 / rms;

        // Scale by inverse RMS and learned weight
        for (val, &w) in x.iter_mut().zip(weight.iter()) {
            *val = (*val * inv_rms) * w;
        }

        Ok(())
    }

    /// Fused forward: x = x + residual, then apply RMSNorm and store in output.
    pub fn forward_fused_residual(
        &self,
        x: &[f32],
        residual: &mut [f32],
        weight: &[f32],
        output: &mut [f32],
    ) -> Result<(), OxideError> {
        let dim = x.len();
        if residual.len() != dim || weight.len() != dim || output.len() != dim {
            return Err(OxideError::Engine(
                "Dimension mismatch in fused residual RMSNorm".to_string(),
            ));
        }

        // 1. Accumulate residual in-place: residual += x
        let mut sum_sq = 0.0f32;
        for i in 0..dim {
            residual[i] += x[i];
            sum_sq += residual[i] * residual[i];
        }

        // 2. Compute RMS
        let rms = (sum_sq / dim as f32 + self.eps).sqrt();
        let inv_rms = 1.0 / rms;

        // 3. Normalize into output
        for i in 0..dim {
            output[i] = (residual[i] * inv_rms) * weight[i];
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmsnorm_forward() {
        let op = FusedRmsNormOp::new(1e-5);
        let mut x = [2.0f32, 2.0, 2.0, 2.0];
        let weights = [1.0f32, 1.0, 1.0, 1.0];

        // RMS of [2, 2, 2, 2] is sqrt(4) = 2.
        // Result should be [1, 1, 1, 1]
        op.forward(&mut x, &weights).unwrap();
        for &v in &x {
            assert!((v - 1.0).abs() < 1e-4);
        }
    }

    #[test]
    fn test_rmsnorm_fused_residual() {
        let op = FusedRmsNormOp::new(1e-5);
        let x = [1.0f32, 1.0, 1.0, 1.0];
        let mut residual = [1.0f32, 1.0, 1.0, 1.0];
        let weights = [0.5f32, 0.5, 0.5, 0.5];
        let mut output = [0.0f32; 4];

        // residual becomes [2, 2, 2, 2], normalized to [1, 1, 1, 1], multiplied by 0.5 -> [0.5, 0.5, 0.5, 0.5]
        op.forward_fused_residual(&x, &mut residual, &weights, &mut output)
            .unwrap();

        assert_eq!(residual, [2.0, 2.0, 2.0, 2.0]);
        for &v in &output {
            assert!((v - 0.5).abs() < 1e-4);
        }
    }
}
