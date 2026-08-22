use ndarray::Array2;
use std::f32::consts::PI;

pub struct SimplexRotaryEncoding {
    pub dimension: usize,
    pub num_agents: usize,
}

impl SimplexRotaryEncoding {
    pub fn new(dimension: usize, num_agents: usize) -> Self {
        Self { dimension, num_agents }
    }

    /// Compute the SRAE angles representing regular simplex vertices
    pub fn compute_simplex_angles(&self) -> Array2<f32> {
        let n = self.num_agents;
        let d = self.dimension;
        let mut simplex_coords = Array2::<f32>::zeros((n, d));

        // Generate simplex coordinates on a hypersphere
        for i in 0..n {
            let angle = (2.0 * PI * i as f32) / n as f32;
            for j in (0..d).step_by(2) {
                if j + 1 < d {
                    simplex_coords[[i, j]] = (angle * (j as f32 + 1.0)).cos();
                    simplex_coords[[i, j + 1]] = (angle * (j as f32 + 1.0)).sin();
                }
            }
        }
        simplex_coords
    }
}
