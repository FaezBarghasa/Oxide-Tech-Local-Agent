use crate::voxelizer::VoxelGrid;
use rayon::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VoxelDiffMap {
    pub false_positive_count: usize, // Voxels in generated but not target
    pub false_negative_count: usize, // Voxels in target but not generated
    pub intersection_count: usize,
    pub union_count: usize,
    pub iou: f64,
}

/// Rayon-accelerated multi-core 3D IoU calculation between two sparse voxel grids.
pub fn compute_iou(generated: &VoxelGrid, target: &VoxelGrid) -> f64 {
    if generated.occupied.is_empty() && target.occupied.is_empty() {
        return 1.0;
    }
    if generated.occupied.is_empty() || target.occupied.is_empty() {
        return 0.0;
    }

    // Parallel intersection count over the smaller set for performance
    let (smaller, larger) = if generated.occupied.len() < target.occupied.len() {
        (&generated.occupied, &target.occupied)
    } else {
        (&target.occupied, &generated.occupied)
    };

    let intersection_count = smaller
        .par_iter()
        .filter(|vox| larger.contains(vox))
        .count();

    let union_count = generated.occupied.len() + target.occupied.len() - intersection_count;

    if union_count == 0 {
        return 0.0;
    }

    intersection_count as f64 / union_count as f64
}

/// Generates a comprehensive 3D difference map for closed-loop VLM agent self-correction.
pub fn generate_diff_map(generated: &VoxelGrid, target: &VoxelGrid) -> VoxelDiffMap {
    let intersection_count = generated
        .occupied
        .par_iter()
        .filter(|vox| target.occupied.contains(vox))
        .count();

    let union_count = generated.occupied.len() + target.occupied.len() - intersection_count;
    let false_positives = generated.occupied.len() - intersection_count;
    let false_negatives = target.occupied.len() - intersection_count;

    let iou = if union_count == 0 {
        0.0
    } else {
        intersection_count as f64 / union_count as f64
    };

    VoxelDiffMap {
        false_positive_count: false_positives,
        false_negative_count: false_negatives,
        intersection_count,
        union_count,
        iou,
    }
}
