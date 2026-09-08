use crate::dsl::{CadScript, Primitive};
use glam::{IVec3, Vec3};
use rayon::prelude::*;
use std::collections::HashSet;

/// High-performance Sparse Voxel Grid using 3D integer coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct VoxelGrid {
    pub occupied: HashSet<IVec3>,
    pub resolution: f32, // Size of each voxel in world units (e.g. 0.1 mm)
}

impl VoxelGrid {
    pub fn new(resolution: f32) -> Self {
        Self {
            occupied: HashSet::new(),
            resolution,
        }
    }

    /// Check if a world point lies inside a primitive
    #[inline(always)]
    fn is_point_inside(primitive: &Primitive, pt: Vec3) -> bool {
        match primitive {
            Primitive::Box { center, size } => {
                let half = *size * 0.5;
                let min = *center - half;
                let max = *center + half;
                pt.x >= min.x
                    && pt.x <= max.x
                    && pt.y >= min.y
                    && pt.y <= max.y
                    && pt.z >= min.z
                    && pt.z <= max.z
            }
            Primitive::Cylinder {
                center,
                radius,
                height,
            } => {
                let half_h = *height * 0.5;
                if pt.z < center.z - half_h || pt.z > center.z + half_h {
                    return false;
                }
                let dx = pt.x - center.x;
                let dy = pt.y - center.y;
                (dx * dx + dy * dy) <= (radius * radius)
            }
            Primitive::Sphere { center, radius } => {
                pt.distance_squared(*center) <= (radius * radius)
            }
        }
    }

    /// Voxelize a box volume in parallel using Rayon
    pub fn from_box(center: Vec3, size: Vec3, resolution: f32) -> Self {
        let prim = Primitive::Box { center, size };
        Self::from_primitive(&prim, resolution)
    }

    /// Voxelize an individual primitive with Rayon parallel scanning
    pub fn from_primitive(primitive: &Primitive, resolution: f32) -> Self {
        let (min, max) = match primitive {
            Primitive::Box { center, size } => {
                let half = *size * 0.5;
                (*center - half, *center + half)
            }
            Primitive::Cylinder {
                center,
                radius,
                height,
            } => {
                let half_h = *height * 0.5;
                (
                    Vec3::new(center.x - radius, center.y - radius, center.z - half_h),
                    Vec3::new(center.x + radius, center.y + radius, center.z + half_h),
                )
            }
            Primitive::Sphere { center, radius } => (
                *center - Vec3::splat(*radius),
                *center + Vec3::splat(*radius),
            ),
        };

        let min_vox = (min / resolution).floor().as_ivec3();
        let max_vox = (max / resolution).ceil().as_ivec3();

        let z_range: Vec<i32> = (min_vox.z..=max_vox.z).collect();

        // Parallel scan across Z slices
        let occupied_slices: Vec<Vec<IVec3>> = z_range
            .par_iter()
            .map(|&z| {
                let mut slice = Vec::new();
                for y in min_vox.y..=max_vox.y {
                    for x in min_vox.x..=max_vox.x {
                        let pt = Vec3::new(x as f32, y as f32, z as f32) * resolution;
                        if Self::is_point_inside(primitive, pt) {
                            slice.push(IVec3::new(x, y, z));
                        }
                    }
                }
                slice
            })
            .collect();

        let mut occupied = HashSet::new();
        for slice in occupied_slices {
            occupied.extend(slice);
        }

        Self {
            occupied,
            resolution,
        }
    }

    /// Voxelize a full CAD script with CSG boolean operations
    pub fn from_script(script: &CadScript, resolution: f32) -> Self {
        if script.primitives.is_empty() {
            return Self::new(resolution);
        }

        // Voxelize all primitives
        let mut primitive_grids: Vec<HashSet<IVec3>> = script
            .primitives
            .iter()
            .map(|p| Self::from_primitive(p, resolution).occupied)
            .collect();

        // Apply Boolean operations in order
        for op in &script.operations {
            match *op {
                crate::dsl::BoolOp::Union(a, b) => {
                    if a < primitive_grids.len() && b < primitive_grids.len() && a != b {
                        let b_set = primitive_grids[b].clone();
                        primitive_grids[a].extend(b_set);
                    }
                }
                crate::dsl::BoolOp::Subtract(target, tool) => {
                    if target < primitive_grids.len()
                        && tool < primitive_grids.len()
                        && target != tool
                    {
                        let tool_set = primitive_grids[tool].clone();
                        primitive_grids[target].retain(|vox| !tool_set.contains(vox));
                    }
                }
                crate::dsl::BoolOp::Intersect(a, b) => {
                    if a < primitive_grids.len() && b < primitive_grids.len() && a != b {
                        let b_set = primitive_grids[b].clone();
                        primitive_grids[a].retain(|vox| b_set.contains(vox));
                    }
                }
            }
        }

        Self {
            occupied: primitive_grids.into_iter().next().unwrap_or_default(),
            resolution,
        }
    }
}
