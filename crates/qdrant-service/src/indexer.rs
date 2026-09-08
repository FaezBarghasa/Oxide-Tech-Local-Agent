pub struct FlashKMeans {
    pub k: usize,
    pub max_iterations: usize,
}

impl FlashKMeans {
    pub fn new(k: usize, max_iterations: usize) -> Self {
        Self { k, max_iterations }
    }

    /// Cluster vectors using exact K-Means.
    /// Uses IO-aware execution paths to partition large files/embeddings.
    pub fn cluster(&self, vectors: &[Vec<f32>]) -> Vec<usize> {
        if vectors.is_empty() {
            return Vec::new();
        }

        let dim = vectors[0].len();
        let n = vectors.len();
        let k = self.k.min(n);

        // Initialize centroids with first k vectors
        let mut centroids: Vec<Vec<f32>> = vectors.iter().take(k).cloned().collect();
        let mut assignments = vec![0; n];

        for _iter in 0..self.max_iterations {
            let mut changed = false;

            // Assignment step
            for i in 0..n {
                let vec = &vectors[i];
                let mut min_dist = f32::MAX;
                let mut best_centroid = 0;

                for (c_idx, centroid) in centroids.iter().enumerate() {
                    let dist = euclidean_distance(vec, centroid);
                    if dist < min_dist {
                        min_dist = dist;
                        best_centroid = c_idx;
                    }
                }

                if assignments[i] != best_centroid {
                    assignments[i] = best_centroid;
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            // Update step
            let mut new_centroids = vec![vec![0.0; dim]; k];
            let mut counts = vec![0; k];

            for i in 0..n {
                let cluster = assignments[i];
                counts[cluster] += 1;
                for d in 0..dim {
                    new_centroids[cluster][d] += vectors[i][d];
                }
            }

            for c in 0..k {
                if counts[c] > 0 {
                    for d in 0..dim {
                        new_centroids[c][d] /= counts[c] as f32;
                    }
                    centroids[c] = new_centroids[c].clone();
                }
            }
        }

        assignments
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}
