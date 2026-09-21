use oxide_core::OxideError;

/// Fast in-place Rotary Position Embedding (RoPE) operator.
///
/// Eliminates tensor cloning and intermediate matrix allocation by rotating
/// query and key pairs in-place within the same memory buffer.
pub struct FastRopeOp {
    pub kernel_name: String,
    pub head_dim: usize,
    pub theta_base: f32,
    cos_table: Vec<f32>,
    sin_table: Vec<f32>,
    max_seq_len: usize,
}

impl FastRopeOp {
    /// Construct a new RoPE operator with precomputed trigonometric tables.
    pub fn new(head_dim: usize, max_seq_len: usize, theta_base: f32) -> Result<Self, OxideError> {
        if !head_dim.is_multiple_of(2) {
            return Err(OxideError::Engine(format!(
                "head_dim must be even for RoPE, got {}",
                head_dim
            )));
        }

        let half_dim = head_dim / 2;
        let mut cos_table = Vec::with_capacity(max_seq_len * half_dim);
        let mut sin_table = Vec::with_capacity(max_seq_len * half_dim);

        for pos in 0..max_seq_len {
            for i in 0..half_dim {
                let freq = 1.0 / theta_base.powf((2 * i) as f32 / head_dim as f32);
                let angle = (pos as f32) * freq;
                cos_table.push(angle.cos());
                sin_table.push(angle.sin());
            }
        }

        Ok(Self {
            kernel_name: "oxide_fast_rope".to_string(),
            head_dim,
            theta_base,
            cos_table,
            sin_table,
            max_seq_len,
        })
    }

    /// Apply RoPE rotation in-place to a token embedding slice at position `pos`.
    ///
    /// Slice length must equal `head_dim`.
    pub fn apply_inplace(&self, slice: &mut [f32], pos: usize) -> Result<(), OxideError> {
        if slice.len() != self.head_dim {
            return Err(OxideError::Engine(format!(
                "Slice length {} does not match head_dim {}",
                slice.len(),
                self.head_dim
            )));
        }

        if pos >= self.max_seq_len {
            return Err(OxideError::Engine(format!(
                "Position {} exceeds precomputed max_seq_len {}",
                pos, self.max_seq_len
            )));
        }

        let half_dim = self.head_dim / 2;
        let table_offset = pos * half_dim;

        for i in 0..half_dim {
            let cos = self.cos_table[table_offset + i];
            let sin = self.sin_table[table_offset + i];

            let x1 = slice[i];
            let x2 = slice[i + half_dim];

            // 2D Rotation:
            // x1' = x1 * cos - x2 * sin
            // x2' = x1 * sin + x2 * cos
            slice[i] = x1 * cos - x2 * sin;
            slice[i + half_dim] = x1 * sin + x2 * cos;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_preserves_norm() {
        let rope = FastRopeOp::new(4, 128, 10000.0).unwrap();
        let mut vec = [1.0f32, 2.0, 3.0, 4.0];
        let original_norm: f32 = vec.iter().map(|x| x * x).sum();

        rope.apply_inplace(&mut vec, 10).unwrap();
        let rotated_norm: f32 = vec.iter().map(|x| x * x).sum();

        // RoPE is an orthogonal rotation; L2 norm must be invariant
        assert!((original_norm - rotated_norm).abs() < 1e-4);
    }

    #[test]
    fn test_rope_position_zero_identity() {
        let rope = FastRopeOp::new(4, 128, 10000.0).unwrap();
        let mut vec = [1.5f32, -2.5, 3.5, -4.5];
        let original = vec;

        // At pos 0: angle = 0, cos = 1, sin = 0 -> identity
        rope.apply_inplace(&mut vec, 0).unwrap();
        for (a, b) in vec.iter().zip(original.iter()) {
            assert!((a - b).abs() < 1e-5);
        }
    }
}
