use crate::ParametricError;
use crate::dag::{CadOperation, FeatureDAG};
use crate::patcher::DeltaPatch;

/// Macro codegen for crystallizing parametric CAD edits and construction history into Rust macros.
pub struct MacroCodegen;

impl MacroCodegen {
    /// Synthesizes a `DeltaPatch` or sequence of `CadOperation`s into a Rust `nexus_macro::parametric_edit!` invocation string.
    pub fn generate_macro(
        dag: &FeatureDAG,
        patches: &[DeltaPatch],
    ) -> Result<String, ParametricError> {
        let mut buffer = String::new();
        buffer.push_str("//! Auto-generated Parametric CAD Macro crystallization\n");
        buffer.push_str("//! Synthesized by parametric-forge engine\n\n");
        buffer.push_str("nexus_macro::parametric_edit! {\n");

        // Write DAG operations
        buffer.push_str("    features: [\n");
        let sorted_indices = dag.topological_order()?;
        for idx in sorted_indices {
            let op = &dag.graph[idx];
            match op {
                CadOperation::Sketch2D {
                    id,
                    name,
                    plane,
                    points,
                    entities,
                    ..
                } => {
                    buffer.push_str(&format!(
                        "        sketch(id: \"{}\", name: \"{}\", plane: {:?}, points_count: {}, entities_count: {}),\n",
                        id,
                        name,
                        plane,
                        points.len(),
                        entities.len()
                    ));
                }
                CadOperation::Extrude {
                    id,
                    name,
                    profile_id,
                    distance,
                    direction,
                } => {
                    buffer.push_str(&format!(
                        "        extrude(id: \"{}\", name: \"{}\", profile_id: \"{}\", distance: {:.4}, dir: {:?}),\n",
                        id, name, profile_id, distance, direction
                    ));
                }
                CadOperation::Fillet {
                    id,
                    name,
                    target_op_id,
                    target_edges,
                    radius,
                } => {
                    buffer.push_str(&format!(
                        "        fillet(id: \"{}\", name: \"{}\", target: \"{}\", edges: {:?}, radius: {:.4}),\n",
                        id, name, target_op_id, target_edges, radius
                    ));
                }
                CadOperation::Chamfer {
                    id,
                    name,
                    target_op_id,
                    target_edges,
                    distance,
                } => {
                    buffer.push_str(&format!(
                        "        chamfer(id: \"{}\", name: \"{}\", target: \"{}\", edges: {:?}, dist: {:.4}),\n",
                        id, name, target_op_id, target_edges, distance
                    ));
                }
                CadOperation::Boolean {
                    id,
                    name,
                    boolean_op,
                    target_a,
                    target_b,
                } => {
                    buffer.push_str(&format!(
                        "        boolean(id: \"{}\", name: \"{}\", op: {:?}, a: \"{}\", b: \"{}\"),\n",
                        id, name, boolean_op, target_a, target_b
                    ));
                }
            }
        }
        buffer.push_str("    ],\n");

        // Write delta patches
        buffer.push_str("    delta_patches: [\n");
        for patch in patches {
            buffer.push_str(&format!(
                "        patch(target: \"{}\", level: {:?}, mutation: {:?}),\n",
                patch.target_node, patch.hierarchy_level, patch.mutation
            ));
        }
        buffer.push_str("    ]\n");
        buffer.push_str("}\n");

        Ok(buffer)
    }
}
