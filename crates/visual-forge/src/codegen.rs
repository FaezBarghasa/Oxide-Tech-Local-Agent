use crate::VisualForgeError;

/// Macro codegen for crystallizing converged visual feedback loops into Rust macros.
pub struct VisualMatchCodegen;

impl VisualMatchCodegen {
    /// Synthesizes a verified closed-loop CAD convergence result into a `nexus_macro::visual_match!` macro string.
    pub fn generate_macro(
        target_name: &str,
        tolerance_mm: f32,
        iou: f64,
        steps: usize,
    ) -> Result<String, VisualForgeError> {
        let mut buffer = String::new();
        buffer.push_str("//! Auto-generated Visual Feedback Macro crystallization\n");
        buffer.push_str("//! Synthesized by visual-forge closed-loop engine\n\n");
        buffer.push_str("nexus_macro::visual_match! {\n");
        buffer.push_str(&format!("    target: \"{}\",\n", target_name));
        buffer.push_str(&format!("    tolerance_mm: {:.4},\n", tolerance_mm));
        buffer.push_str(&format!("    volumetric_iou: {:.4},\n", iou));
        buffer.push_str(&format!("    convergence_steps: {},\n", steps));
        buffer.push_str("    status: \"CONVERGED_VERIFIED\"\n");
        buffer.push_str("}\n");

        Ok(buffer)
    }
}
