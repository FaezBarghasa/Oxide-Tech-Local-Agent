use glam::Vec3;
use parametric_forge::{BooleanType, CadOperation, Plane};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use crate::sdf_feedback::SDFVolume;

/// Recoverable error returned by the Shadow Kernel when an invalid CAD operation is detected.
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KernelError {
    #[error("Non-manifold intersection or topological failure: {0}")]
    NonManifoldIntersection(String),

    #[error("Invalid or unclosed sketch profile: {0}")]
    InvalidProfile(String),

    #[error("Degenerate feature or zero volume: {0}")]
    DegenerateFeature(String),

    #[error("Feature parameter out of bounds: {0}")]
    OutOfBoundsFeature(String),

    #[error("Target operation '{0}' not found in shadow kernel state")]
    TargetNotFound(String),

    #[error("Evaluation error: {0}")]
    EvaluationFailed(String),
}

/// In-memory representation of an evaluated solid inside the Shadow Kernel sandbox.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolidRepresentation {
    pub id: Uuid,
    pub name: String,
    pub sdf: SDFVolume,
    pub bounds: [Vec3; 2],
    pub estimated_volume_mm3: f64,
}

/// Crash-proof Shadow Kernel State Machine executing parametric CAD sequences in memory.
#[derive(Debug, Clone)]
pub struct ShadowKernel {
    pub solids: HashMap<Uuid, SolidRepresentation>,
    pub sketch_profiles: HashMap<Uuid, (Vec<[f32; 2]>, Plane)>,
    pub current_solid_id: Option<Uuid>,
    pub default_bounds: [Vec3; 2],
    pub default_resolution: f32,
}

impl Default for ShadowKernel {
    fn default() -> Self {
        Self::new([-50.0, -50.0, -50.0], [50.0, 50.0, 50.0], 1.0)
    }
}

impl ShadowKernel {
    /// Creates a new Shadow Kernel with specified evaluation bounding box and voxel resolution.
    pub fn new(min_bound: [f32; 3], max_bound: [f32; 3], resolution: f32) -> Self {
        Self {
            solids: HashMap::new(),
            sketch_profiles: HashMap::new(),
            current_solid_id: None,
            default_bounds: [Vec3::from_array(min_bound), Vec3::from_array(max_bound)],
            default_resolution: resolution,
        }
    }

    /// Returns the currently active solid representation, if any.
    pub fn current_solid(&self) -> Option<&SolidRepresentation> {
        self.current_solid_id.and_then(|id| self.solids.get(&id))
    }

    /// Evaluates a parametric CAD operation safely, catching topological or geometric errors.
    pub fn evaluate_operation(
        &mut self,
        op: &CadOperation,
    ) -> Result<&SolidRepresentation, KernelError> {
        match op {
            CadOperation::Sketch2D {
                id,
                name: _,
                plane,
                points,
                ..
            } => {
                if points.len() < 3 {
                    return Err(KernelError::InvalidProfile(
                        "Sketch must contain at least 3 points to form a closed polygon"
                            .to_string(),
                    ));
                }

                let poly_pts: Vec<[f32; 2]> =
                    points.iter().map(|p| [p[0] as f32, p[1] as f32]).collect();

                // Check for degenerate collinear points
                let mut area = 0.0f32;
                let n = poly_pts.len();
                for i in 0..n {
                    let j = (i + 1) % n;
                    area += poly_pts[i][0] * poly_pts[j][1] - poly_pts[j][0] * poly_pts[i][1];
                }
                if area.abs() < 1e-4 {
                    return Err(KernelError::DegenerateFeature(
                        "Sketch points are collinear or define zero area".to_string(),
                    ));
                }

                self.sketch_profiles.insert(*id, (poly_pts, *plane));

                // Return current solid if one exists, otherwise empty state representation
                if let Some(curr_id) = self.current_solid_id {
                    self.solids.get(&curr_id).ok_or_else(|| {
                        KernelError::EvaluationFailed("Corrupted active solid state".to_string())
                    })
                } else {
                    // Create an initial empty representation for sketch
                    let empty_sdf =
                        SDFVolume::new_empty(self.default_bounds, self.default_resolution);
                    let rep = SolidRepresentation {
                        id: *id,
                        name: "sketch_placeholder".to_string(),
                        sdf: empty_sdf,
                        bounds: self.default_bounds,
                        estimated_volume_mm3: 0.0,
                    };
                    self.solids.insert(*id, rep);
                    self.current_solid_id = Some(*id);
                    Ok(self.solids.get(id).unwrap())
                }
            }

            CadOperation::Extrude {
                id,
                name,
                profile_id,
                distance,
                direction,
            } => {
                if *distance <= 0.0 {
                    return Err(KernelError::OutOfBoundsFeature(format!(
                        "Extrusion depth must be strictly positive, received {:.4}",
                        distance
                    )));
                }

                let (poly_pts, _plane) = self
                    .sketch_profiles
                    .get(profile_id)
                    .ok_or_else(|| KernelError::TargetNotFound(profile_id.to_string()))?;

                let dir_vec = Vec3::new(
                    direction[0] as f32,
                    direction[1] as f32,
                    direction[2] as f32,
                );
                if dir_vec.length_squared() < 1e-6 {
                    return Err(KernelError::DegenerateFeature(
                        "Extrusion direction vector must be non-zero".to_string(),
                    ));
                }

                let z_min = 0.0f32;
                let z_max = *distance as f32;

                let sdf = SDFVolume::from_extruded_polygon(
                    poly_pts,
                    z_min,
                    z_max,
                    self.default_bounds,
                    self.default_resolution,
                );

                // Estimate volume from voxels
                let inside_count = sdf.voxels.iter().filter(|&&d| d <= 0.0).count();
                let vol = inside_count as f64 * sdf.voxel_volume();

                let rep = SolidRepresentation {
                    id: *id,
                    name: name.clone(),
                    sdf,
                    bounds: self.default_bounds,
                    estimated_volume_mm3: vol,
                };

                self.solids.insert(*id, rep);
                self.current_solid_id = Some(*id);
                Ok(self.solids.get(id).unwrap())
            }

            CadOperation::Fillet {
                id,
                name,
                target_op_id,
                radius,
                ..
            } => {
                if *radius <= 0.0 {
                    return Err(KernelError::OutOfBoundsFeature(format!(
                        "Fillet radius must be strictly positive, received {:.4}",
                        radius
                    )));
                }

                let target = self
                    .solids
                    .get(target_op_id)
                    .ok_or_else(|| KernelError::TargetNotFound(target_op_id.to_string()))?;

                if target.estimated_volume_mm3 <= 0.0 {
                    return Err(KernelError::DegenerateFeature(
                        "Cannot apply fillet to empty or zero-volume solid".to_string(),
                    ));
                }

                // In SDF domain, fillet is represented as smooth rounding
                let mut filleted_sdf = target.sdf.clone();
                let r_f32 = *radius as f32;
                filleted_sdf.voxels.iter_mut().for_each(|d| {
                    if *d > -r_f32 && *d < r_f32 {
                        *d = (*d * 0.9).copysign(*d);
                    }
                });

                let inside_count = filleted_sdf.voxels.iter().filter(|&&d| d <= 0.0).count();
                let vol = inside_count as f64 * filleted_sdf.voxel_volume();

                let rep = SolidRepresentation {
                    id: *id,
                    name: name.clone(),
                    sdf: filleted_sdf,
                    bounds: target.bounds,
                    estimated_volume_mm3: vol,
                };

                self.solids.insert(*id, rep);
                self.current_solid_id = Some(*id);
                Ok(self.solids.get(id).unwrap())
            }

            CadOperation::Chamfer {
                id,
                name,
                target_op_id,
                distance,
                ..
            } => {
                if *distance <= 0.0 {
                    return Err(KernelError::OutOfBoundsFeature(format!(
                        "Chamfer distance must be strictly positive, received {:.4}",
                        distance
                    )));
                }

                let target = self
                    .solids
                    .get(target_op_id)
                    .ok_or_else(|| KernelError::TargetNotFound(target_op_id.to_string()))?;

                let mut chamfered_sdf = target.sdf.clone();
                let dist_f32 = *distance as f32;
                chamfered_sdf.voxels.iter_mut().for_each(|d| {
                    if *d > -dist_f32 && *d < dist_f32 {
                        *d = (*d * 0.85).copysign(*d);
                    }
                });

                let inside_count = chamfered_sdf.voxels.iter().filter(|&&d| d <= 0.0).count();
                let vol = inside_count as f64 * chamfered_sdf.voxel_volume();

                let rep = SolidRepresentation {
                    id: *id,
                    name: name.clone(),
                    sdf: chamfered_sdf,
                    bounds: target.bounds,
                    estimated_volume_mm3: vol,
                };

                self.solids.insert(*id, rep);
                self.current_solid_id = Some(*id);
                Ok(self.solids.get(id).unwrap())
            }

            CadOperation::Boolean {
                id,
                name,
                boolean_op,
                target_a,
                target_b,
            } => {
                let solid_a = self
                    .solids
                    .get(target_a)
                    .ok_or_else(|| KernelError::TargetNotFound(target_a.to_string()))?;

                let solid_b = self
                    .solids
                    .get(target_b)
                    .ok_or_else(|| KernelError::TargetNotFound(target_b.to_string()))?;

                let result_sdf = match boolean_op {
                    BooleanType::Union => solid_a.sdf.boolean_union(&solid_b.sdf),
                    BooleanType::Difference => solid_a.sdf.boolean_difference(&solid_b.sdf),
                    BooleanType::Intersection => {
                        let inter = solid_a.sdf.boolean_intersection(&solid_b.sdf);
                        let inside_count = inter.voxels.iter().filter(|&&d| d <= 0.0).count();
                        if inside_count == 0 {
                            return Err(KernelError::NonManifoldIntersection(format!(
                                "Boolean intersection between '{}' and '{}' produced disjoint / empty volume",
                                solid_a.name, solid_b.name
                            )));
                        }
                        inter
                    }
                };

                let inside_count = result_sdf.voxels.iter().filter(|&&d| d <= 0.0).count();
                let vol = inside_count as f64 * result_sdf.voxel_volume();

                let rep = SolidRepresentation {
                    id: *id,
                    name: name.clone(),
                    sdf: result_sdf,
                    bounds: solid_a.bounds,
                    estimated_volume_mm3: vol,
                };

                self.solids.insert(*id, rep);
                self.current_solid_id = Some(*id);
                Ok(self.solids.get(id).unwrap())
            }
        }
    }
}
