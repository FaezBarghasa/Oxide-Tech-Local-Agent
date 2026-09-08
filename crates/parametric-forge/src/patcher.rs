use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constraint_solver::GeometricConstraint;
use crate::dag::{CadOperation, FeatureDAG, Plane};
use crate::ParametricError;

/// The target level in the CAD hierarchy being modified by a delta patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyLevel {
    Curve,
    Loop,
    Sketch,
    Extrusion,
    Fillet,
    Chamfer,
    Boolean,
}

/// A specific mutation applied to a CAD operation node in the Feature DAG.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation_type", rename_all = "snake_case")]
pub enum Mutation {
    UpdateExtrusionDepth(f64),
    UpdateExtrusionDirection([f64; 3]),
    AddConstraint(GeometricConstraint),
    RemoveConstraint(usize),
    UpdateFilletRadius(f64),
    UpdateChamferDistance(f64),
    UpdatePlane(Plane),
    UpdatePointPosition {
        point_idx: usize,
        position: [f64; 2],
    },
}

/// A fine-grained delta patch that modifies only a masked hierarchy level without regenerating full history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeltaPatch {
    pub target_node: Uuid,
    pub hierarchy_level: HierarchyLevel,
    pub mutation: Mutation,
}

impl DeltaPatch {
    pub fn new(target_node: Uuid, hierarchy_level: HierarchyLevel, mutation: Mutation) -> Self {
        Self {
            target_node,
            hierarchy_level,
            mutation,
        }
    }
}

impl FeatureDAG {
    /// Applies a hierarchy-aware `DeltaPatch` directly to the target node in the Feature DAG.
    pub fn apply_patch(&mut self, patch: &DeltaPatch) -> Result<(), ParametricError> {
        let node_idx = self.find_node_by_uuid(patch.target_node)?;
        let op = &mut self.graph[node_idx];

        match (&patch.hierarchy_level, &patch.mutation, op) {
            // 1. Extrusion mutations
            (
                HierarchyLevel::Extrusion,
                Mutation::UpdateExtrusionDepth(new_depth),
                CadOperation::Extrude { distance, .. },
            ) => {
                if *new_depth <= 0.0 {
                    return Err(ParametricError::InvalidParameter(
                        "Extrusion depth must be strictly positive".to_string(),
                    ));
                }
                *distance = *new_depth;
                Ok(())
            }
            (
                HierarchyLevel::Extrusion,
                Mutation::UpdateExtrusionDirection(new_dir),
                CadOperation::Extrude { direction, .. },
            ) => {
                *direction = *new_dir;
                Ok(())
            }

            // 2. Sketch constraints & points
            (
                HierarchyLevel::Sketch,
                Mutation::AddConstraint(constraint),
                CadOperation::Sketch2D { constraints, .. },
            ) => {
                constraints.push(constraint.clone());
                Ok(())
            }
            (
                HierarchyLevel::Sketch,
                Mutation::RemoveConstraint(idx),
                CadOperation::Sketch2D { constraints, .. },
            ) => {
                if *idx < constraints.len() {
                    constraints.remove(*idx);
                    Ok(())
                } else {
                    Err(ParametricError::InvalidParameter(format!(
                        "Constraint index {} out of bounds",
                        idx
                    )))
                }
            }
            (
                HierarchyLevel::Sketch,
                Mutation::UpdatePlane(new_plane),
                CadOperation::Sketch2D { plane, .. },
            ) => {
                *plane = *new_plane;
                Ok(())
            }
            (
                HierarchyLevel::Sketch,
                Mutation::UpdatePointPosition {
                    point_idx,
                    position,
                },
                CadOperation::Sketch2D { points, .. },
            ) => {
                if *point_idx < points.len() {
                    points[*point_idx] = *position;
                    Ok(())
                } else {
                    Err(ParametricError::InvalidParameter(format!(
                        "Point index {} out of bounds",
                        point_idx
                    )))
                }
            }

            // 3. Fillet mutations
            (
                HierarchyLevel::Fillet,
                Mutation::UpdateFilletRadius(new_radius),
                CadOperation::Fillet { radius, .. },
            ) => {
                if *new_radius <= 0.0 {
                    return Err(ParametricError::InvalidParameter(
                        "Fillet radius must be strictly positive".to_string(),
                    ));
                }
                *radius = *new_radius;
                Ok(())
            }

            // 4. Chamfer mutations
            (
                HierarchyLevel::Chamfer,
                Mutation::UpdateChamferDistance(new_dist),
                CadOperation::Chamfer { distance, .. },
            ) => {
                if *new_dist <= 0.0 {
                    return Err(ParametricError::InvalidParameter(
                        "Chamfer distance must be strictly positive".to_string(),
                    ));
                }
                *distance = *new_dist;
                Ok(())
            }

            // Mismatch
            (level, mutation, target_op) => Err(ParametricError::HierarchyMismatch(format!(
                "Cannot apply {:?} mutation at level {:?} to node '{}' ({:?})",
                mutation,
                level,
                target_op.name(),
                target_op.id()
            ))),
        }
    }

    /// Applies a batch of `DeltaPatch` commands in sequence.
    pub fn apply_patches(&mut self, patches: &[DeltaPatch]) -> Result<(), ParametricError> {
        for patch in patches {
            self.apply_patch(patch)?;
        }
        Ok(())
    }
}
