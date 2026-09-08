use glam::{Vec2, Vec3};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// Detailed geometric correction report produced by deterministic 3D volumetric diffing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeometryCorrectionReport {
    /// Volume of material (in mm³) present in target geometry but missing in generated geometry.
    pub missing_material_mm3: f64,
    /// Volume of material (in mm³) present in generated geometry but absent in target geometry.
    pub excess_material_mm3: f64,
    /// Maximum surface deviation distance (in mm) between generated and target models.
    pub max_surface_deviation_mm: f32,
    /// World coordinates (x, y, z) where the maximum geometric deviation is concentrated.
    pub deviation_center: Vec3,
    /// 3D Volumetric Intersection over Union (0.0 to 1.0, where 1.0 is a perfect match).
    pub volumetric_iou: f64,
    /// Structured, token-efficient text prompt hint for the LLM ReAct loop.
    pub correction_hint: String,
}

/// A 3D Volumetric Signed Distance Field (SDF) grid.
///
/// Distances:
/// - `< 0.0`: Inside the solid boundary
/// - `== 0.0`: Exactly on the surface
/// - `> 0.0`: Outside the solid boundary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SDFVolume {
    pub voxels: Vec<f32>,
    pub resolution: f32,
    pub dim: [usize; 3],
    pub bounds: [Vec3; 2],
}

impl SDFVolume {
    /// Creates a new empty SDF volume with all voxels set to infinity (outside).
    pub fn new_empty(bounds: [Vec3; 2], resolution: f32) -> Self {
        let size = bounds[1] - bounds[0];
        let nx = ((size.x / resolution).ceil() as usize).max(1);
        let ny = ((size.y / resolution).ceil() as usize).max(1);
        let nz = ((size.z / resolution).ceil() as usize).max(1);
        let count = nx * ny * nz;

        Self {
            voxels: vec![f32::INFINITY; count],
            resolution,
            dim: [nx, ny, nz],
            bounds,
        }
    }

    /// Number of voxels in the grid.
    #[inline]
    pub fn total_voxels(&self) -> usize {
        self.voxels.len()
    }

    /// Volume of a single voxel in mm³.
    #[inline]
    pub fn voxel_volume(&self) -> f64 {
        (self.resolution as f64).powi(3)
    }

    /// Converts 3D grid indices (gx, gy, gz) into a 1D flat index.
    #[inline]
    pub fn grid_to_index(&self, gx: usize, gy: usize, gz: usize) -> usize {
        gz * (self.dim[0] * self.dim[1]) + gy * self.dim[0] + gx
    }

    /// Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self.
    #[inline]
    pub fn static_index_to_grid(idx: usize, dim: [usize; 3]) -> [usize; 3] {
        let area = dim[0] * dim[1];
        let gz = idx / area;
        let rem = idx % area;
        let gy = rem / dim[0];
        let gx = rem % dim[0];
        [gx, gy, gz]
    }

    /// Converts 3D grid coordinates to world-space coordinates (voxel center) statically.
    #[inline]
    pub fn static_grid_to_world(
        gx: usize,
        gy: usize,
        gz: usize,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Vec3 {
        Vec3::new(
            bounds[0].x + (gx as f32 + 0.5) * resolution,
            bounds[0].y + (gy as f32 + 0.5) * resolution,
            bounds[0].z + (gz as f32 + 0.5) * resolution,
        )
    }

    /// Converts a 1D flat index into 3D grid indices (gx, gy, gz).
    #[inline]
    pub fn index_to_grid(&self, idx: usize) -> [usize; 3] {
        Self::static_index_to_grid(idx, self.dim)
    }

    /// Converts 3D grid coordinates to world-space coordinates (voxel center).
    #[inline]
    pub fn grid_to_world(&self, gx: usize, gy: usize, gz: usize) -> Vec3 {
        Self::static_grid_to_world(gx, gy, gz, self.bounds, self.resolution)
    }

    /// Converts a 1D flat index directly to world-space coordinates.
    #[inline]
    pub fn index_to_world(&self, idx: usize) -> Vec3 {
        let [gx, gy, gz] = self.index_to_grid(idx);
        self.grid_to_world(gx, gy, gz)
    }

    /// Converts a world-space point to integer grid coordinates if within bounds.
    pub fn world_to_grid(&self, p: Vec3) -> Option<[usize; 3]> {
        if p.x < self.bounds[0].x
            || p.x > self.bounds[1].x
            || p.y < self.bounds[0].y
            || p.y > self.bounds[1].y
            || p.z < self.bounds[0].z
            || p.z > self.bounds[1].z
        {
            return None;
        }

        let gx = ((p.x - self.bounds[0].x) / self.resolution).floor() as usize;
        let gy = ((p.y - self.bounds[0].y) / self.resolution).floor() as usize;
        let gz = ((p.z - self.bounds[0].z) / self.resolution).floor() as usize;

        if gx < self.dim[0] && gy < self.dim[1] && gz < self.dim[2] {
            Some([gx, gy, gz])
        } else {
            None
        }
    }

    /// Rasterizes an analytical axis-aligned Box primitive into an SDF volume.
    pub fn from_box(center: Vec3, size: Vec3, bounds: [Vec3; 2], resolution: f32) -> Self {
        let mut volume = Self::new_empty(bounds, resolution);
        let half_size = size * 0.5;
        let dim = volume.dim;

        volume
            .voxels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, v)| {
                let [gx, gy, gz] = Self::static_index_to_grid(idx, dim);
                let p = Self::static_grid_to_world(gx, gy, gz, bounds, resolution);
                let d = (p - center).abs() - half_size;
                let outside_dist = d.max(Vec3::ZERO).length();
                let inside_dist = d.x.max(d.y.max(d.z)).min(0.0);
                *v = outside_dist + inside_dist;
            });

        volume
    }

    /// Rasterizes an analytical Sphere primitive into an SDF volume.
    pub fn from_sphere(center: Vec3, radius: f32, bounds: [Vec3; 2], resolution: f32) -> Self {
        let mut volume = Self::new_empty(bounds, resolution);
        let dim = volume.dim;

        volume
            .voxels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, v)| {
                let [gx, gy, gz] = Self::static_index_to_grid(idx, dim);
                let p = Self::static_grid_to_world(gx, gy, gz, bounds, resolution);
                *v = (p - center).length() - radius;
            });

        volume
    }

    /// Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.
    pub fn from_cylinder(
        center: Vec3,
        radius: f32,
        height: f32,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Self {
        let mut volume = Self::new_empty(bounds, resolution);
        let half_h = height * 0.5;
        let dim = volume.dim;

        volume
            .voxels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, v)| {
                let [gx, gy, gz] = Self::static_index_to_grid(idx, dim);
                let p = Self::static_grid_to_world(gx, gy, gz, bounds, resolution);
                let offset = p - center;

                let d_xy = Vec2::new(offset.x, offset.y).length() - radius;
                let d_z = offset.z.abs() - half_h;

                let outside_xy = d_xy.max(0.0);
                let outside_z = d_z.max(0.0);
                let outside_dist = Vec2::new(outside_xy, outside_z).length();
                let inside_dist = d_xy.max(d_z).min(0.0);

                *v = outside_dist + inside_dist;
            });

        volume
    }

    /// Rasterizes an extruded 2D convex/simple polygon into an SDF volume.
    pub fn from_extruded_polygon(
        polygon: &[[f32; 2]],
        z_min: f32,
        z_max: f32,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Self {
        let mut volume = Self::new_empty(bounds, resolution);
        let half_h = (z_max - z_min) * 0.5;
        let z_center = (z_min + z_max) * 0.5;
        let dim = volume.dim;

        volume
            .voxels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, v)| {
                let [gx, gy, gz] = Self::static_index_to_grid(idx, dim);
                let p = Self::static_grid_to_world(gx, gy, gz, bounds, resolution);

                // Compute 2D polygon signed distance
                let d_2d = Self::sd_polygon_2d(Vec2::new(p.x, p.y), polygon);
                let d_z = (p.z - z_center).abs() - half_h;

                let outside_2d = d_2d.max(0.0);
                let outside_z = d_z.max(0.0);
                let outside_dist = Vec2::new(outside_2d, outside_z).length();
                let inside_dist = d_2d.max(d_z).min(0.0);

                *v = outside_dist + inside_dist;
            });

        volume
    }

    /// Computes exact signed distance from point p to 2D polygon.
    fn sd_polygon_2d(p: Vec2, poly: &[[f32; 2]]) -> f32 {
        let n = poly.len();
        if n < 3 {
            return f32::INFINITY;
        }

        let mut min_dist_sq = f32::INFINITY;
        let mut inside = false;

        for i in 0..n {
            let j = (i + 1) % n;
            let a = Vec2::new(poly[i][0], poly[i][1]);
            let b = Vec2::new(poly[j][0], poly[j][1]);

            // Distance to line segment a->b
            let pa = p - a;
            let ba = b - a;
            let h = (pa.dot(ba) / ba.length_squared()).clamp(0.0, 1.0);
            let dist_sq = (pa - ba * h).length_squared();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
            }

            // Ray-casting for winding/inside test (horizontal ray towards +X)
            if (a.y > p.y) != (b.y > p.y) {
                let intersect_x = (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x;
                if p.x < intersect_x {
                    inside = !inside;
                }
            }
        }

        let d = min_dist_sq.sqrt();
        if inside { -d } else { d }
    }

    /// Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.
    pub fn boolean_union(&self, other: &SDFVolume) -> Self {
        let mut result = self.clone();
        result
            .voxels
            .par_iter_mut()
            .zip(other.voxels.par_iter())
            .for_each(|(r, &o)| {
                *r = r.min(o);
            });
        result
    }

    /// Performs CSG Boolean Difference (A \ B).
    pub fn boolean_difference(&self, other: &SDFVolume) -> Self {
        let mut result = self.clone();
        result
            .voxels
            .par_iter_mut()
            .zip(other.voxels.par_iter())
            .for_each(|(r, &o)| {
                *r = r.max(-o);
            });
        result
    }

    /// Performs CSG Boolean Intersection (A ∩ B).
    pub fn boolean_intersection(&self, other: &SDFVolume) -> Self {
        let mut result = self.clone();
        result
            .voxels
            .par_iter_mut()
            .zip(other.voxels.par_iter())
            .for_each(|(r, &o)| {
                *r = r.max(o);
            });
        result
    }

    /// Polynomial Smooth Boolean Union (fillet-like blending).
    pub fn smooth_union(&self, other: &SDFVolume, k: f32) -> Self {
        let mut result = self.clone();
        result
            .voxels
            .par_iter_mut()
            .zip(other.voxels.par_iter())
            .for_each(|(r, &o)| {
                let h = (0.5 + 0.5 * (o - *r) / k).clamp(0.0, 1.0);
                *r = *r * h + o * (1.0 - h) - k * h * (1.0 - h);
            });
        result
    }

    /// Computes the deterministic 3D volumetric difference between generated CAD and target CAD.
    ///
    /// Returns a structured `GeometryCorrectionReport` that can be directly parsed
    /// by LLM agents without running 2D rendering or VLM visual encoders.
    pub fn compute_volumetric_diff(&self, target: &SDFVolume) -> GeometryCorrectionReport {
        let voxel_vol = self.voxel_volume();

        // Accumulators for Rayon parallel reduction
        struct DiffAcc {
            missing_voxels: usize,
            excess_voxels: usize,
            intersection_voxels: usize,
            union_voxels: usize,
            max_dev: f32,
            dev_idx: usize,
        }

        let len = self.voxels.len().min(target.voxels.len());

        let acc = (0..len)
            .into_par_iter()
            .fold(
                || DiffAcc {
                    missing_voxels: 0,
                    excess_voxels: 0,
                    intersection_voxels: 0,
                    union_voxels: 0,
                    max_dev: 0.0,
                    dev_idx: 0,
                },
                |mut acc, idx| {
                    let gen_d = self.voxels[idx];
                    let tgt_d = target.voxels[idx];

                    let gen_inside = gen_d <= 0.0;
                    let tgt_inside = tgt_d <= 0.0;

                    if tgt_inside && !gen_inside {
                        acc.missing_voxels += 1;
                    }
                    if !tgt_inside && gen_inside {
                        acc.excess_voxels += 1;
                    }
                    if gen_inside && tgt_inside {
                        acc.intersection_voxels += 1;
                    }
                    if gen_inside || tgt_inside {
                        acc.union_voxels += 1;
                    }

                    // Surface deviation: consider voxels near either surface
                    if gen_d.abs() < 2.0 * self.resolution || tgt_d.abs() < 2.0 * self.resolution {
                        let dev = (gen_d - tgt_d).abs();
                        if dev > acc.max_dev {
                            acc.max_dev = dev;
                            acc.dev_idx = idx;
                        }
                    }

                    acc
                },
            )
            .reduce(
                || DiffAcc {
                    missing_voxels: 0,
                    excess_voxels: 0,
                    intersection_voxels: 0,
                    union_voxels: 0,
                    max_dev: 0.0,
                    dev_idx: 0,
                },
                |a, b| {
                    let (max_dev, dev_idx) = if a.max_dev >= b.max_dev {
                        (a.max_dev, a.dev_idx)
                    } else {
                        (b.max_dev, b.dev_idx)
                    };
                    DiffAcc {
                        missing_voxels: a.missing_voxels + b.missing_voxels,
                        excess_voxels: a.excess_voxels + b.excess_voxels,
                        intersection_voxels: a.intersection_voxels + b.intersection_voxels,
                        union_voxels: a.union_voxels + b.union_voxels,
                        max_dev,
                        dev_idx,
                    }
                },
            );

        let missing_vol = acc.missing_voxels as f64 * voxel_vol;
        let excess_vol = acc.excess_voxels as f64 * voxel_vol;
        let iou = if acc.union_voxels > 0 {
            acc.intersection_voxels as f64 / acc.union_voxels as f64
        } else {
            1.0
        };

        let dev_center = self.index_to_world(acc.dev_idx);

        let correction_hint = if iou > 0.995 && acc.max_dev < self.resolution {
            "Volumetric geometry matches target within tolerance (IoU > 99.5%). Model verified."
                .to_string()
        } else if excess_vol > missing_vol && excess_vol > 1e-4 {
            format!(
                "Excess material of {:.2}mm³ concentrated near [{:.2}, {:.2}, {:.2}]. Suggest adding Boolean Difference (subtractive pocket/hole) or increasing Fillet/Chamfer radius.",
                excess_vol, dev_center.x, dev_center.y, dev_center.z
            )
        } else if missing_vol > excess_vol && missing_vol > 1e-4 {
            format!(
                "Missing material of {:.2}mm³ concentrated near [{:.2}, {:.2}, {:.2}]. Suggest extending Extrusion depth or adding additive feature.",
                missing_vol, dev_center.x, dev_center.y, dev_center.z
            )
        } else {
            format!(
                "Surface deviation of {:.2}mm detected at [{:.2}, {:.2}, {:.2}] (IoU: {:.2}%). Adjust parametric dimensions.",
                acc.max_dev,
                dev_center.x,
                dev_center.y,
                dev_center.z,
                iou * 100.0
            )
        };

        GeometryCorrectionReport {
            missing_material_mm3: missing_vol,
            excess_material_mm3: excess_vol,
            max_surface_deviation_mm: acc.max_dev,
            deviation_center: dev_center,
            volumetric_iou: iou,
            correction_hint,
        }
    }
}
