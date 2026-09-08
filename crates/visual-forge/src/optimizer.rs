use serde::{Deserialize, Serialize};

use crate::sdf_feedback::{GeometryCorrectionReport, SDFVolume};

/// Closed-loop visual feedback optimizer guiding parametric adjustments based on volumetric error deltas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualFeedbackOptimizer {
    pub target_sdf: SDFVolume,
    pub tolerance_iou: f64,
    pub max_allowed_deviation_mm: f32,
    pub history: Vec<GeometryCorrectionReport>,
}

impl VisualFeedbackOptimizer {
    /// Creates a new closed-loop optimizer targeting the specified reference SDF volume.
    pub fn new(target_sdf: SDFVolume, tolerance_iou: f64, max_allowed_deviation_mm: f32) -> Self {
        Self {
            target_sdf,
            tolerance_iou,
            max_allowed_deviation_mm,
            history: Vec::new(),
        }
    }

    /// Evaluates the current generated solid state against the target, recording step telemetry.
    pub fn evaluate_step(&mut self, current_sdf: &SDFVolume) -> GeometryCorrectionReport {
        let report = current_sdf.compute_volumetric_diff(&self.target_sdf);
        self.history.push(report.clone());
        report
    }

    /// Checks whether the model has converged to within the target IoU and surface deviation tolerances.
    pub fn is_converged(&self) -> bool {
        if let Some(last) = self.history.last() {
            last.volumetric_iou >= self.tolerance_iou
                && last.max_surface_deviation_mm <= self.max_allowed_deviation_mm
        } else {
            false
        }
    }

    /// Computes convergence progress across iterations.
    /// Returns `(current_iou, is_improving)`.
    pub fn convergence_trend(&self) -> (f64, bool) {
        if self.history.is_empty() {
            return (0.0, false);
        }
        let current_iou = self.history.last().unwrap().volumetric_iou;
        if self.history.len() < 2 {
            return (current_iou, true);
        }
        let prev_iou = self.history[self.history.len() - 2].volumetric_iou;
        (current_iou, current_iou >= prev_iou)
    }

    /// Generates structured suggestions for the next parametric operation or hierarchy delta.
    pub fn suggest_next_action(&self, report: &GeometryCorrectionReport) -> String {
        if self.is_converged() {
            return "Optimization complete: Geometry has converged to target within specified tolerance."
                .to_string();
        }

        if report.excess_material_mm3 > 1.0 {
            format!(
                "REDUCE MATERIAL: Remove {:.2}mm³ at [{:.2}, {:.2}, {:.2}] via Boolean Subtract, Chamfer, or reduced Extrusion depth.",
                report.excess_material_mm3,
                report.deviation_center.x,
                report.deviation_center.y,
                report.deviation_center.z
            )
        } else if report.missing_material_mm3 > 1.0 {
            format!(
                "ADD MATERIAL: Add {:.2}mm³ at [{:.2}, {:.2}, {:.2}] via Extrude addition or Boolean Union.",
                report.missing_material_mm3,
                report.deviation_center.x,
                report.deviation_center.y,
                report.deviation_center.z
            )
        } else {
            format!(
                "FINE TUNE: Adjust control points or fillet radius near [{:.2}, {:.2}, {:.2}] to eliminate {:.2}mm surface deviation.",
                report.deviation_center.x,
                report.deviation_center.y,
                report.deviation_center.z,
                report.max_surface_deviation_mm
            )
        }
    }
}
