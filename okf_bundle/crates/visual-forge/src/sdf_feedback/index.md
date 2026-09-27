# sdf_feedback

## Classs

- [DiffAcc](DiffAcc.md) — Accumulators for Rayon parallel reduction
- [GeometryCorrectionReport](GeometryCorrectionReport.md) — Detailed geometric correction report produced by deterministic 3D volumetric diffing.
- [SDFVolume](SDFVolume.md) — A 3D Volumetric Signed Distance Field (SDF) grid.

## Functions

- [boolean_difference](boolean_difference.md) — Performs CSG Boolean Difference (A \ B).
- [boolean_difference](boolean_difference_1.md) — Performs CSG Boolean Difference (A \ B).
- [boolean_intersection](boolean_intersection.md) — Performs CSG Boolean Intersection (A ∩ B).
- [boolean_intersection](boolean_intersection_1.md) — Performs CSG Boolean Intersection (A ∩ B).
- [boolean_union](boolean_union.md) — Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.
- [boolean_union](boolean_union_1.md) — Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.
- [compute_volumetric_diff](compute_volumetric_diff.md) — Computes the deterministic 3D volumetric difference between generated CAD and target CAD.
- [compute_volumetric_diff](compute_volumetric_diff_1.md) — Computes the deterministic 3D volumetric difference between generated CAD and target CAD.
- [from_box](from_box.md) — Rasterizes an analytical axis-aligned Box primitive into an SDF volume.
- [from_box](from_box_1.md) — Rasterizes an analytical axis-aligned Box primitive into an SDF volume.
- [from_cylinder](from_cylinder.md) — Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.
- [from_cylinder](from_cylinder_1.md) — Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.
- [from_extruded_polygon](from_extruded_polygon.md) — Rasterizes an extruded 2D convex/simple polygon into an SDF volume.
- [from_extruded_polygon](from_extruded_polygon_1.md) — Rasterizes an extruded 2D convex/simple polygon into an SDF volume.
- [from_sphere](from_sphere.md) — Rasterizes an analytical Sphere primitive into an SDF volume.
- [from_sphere](from_sphere_1.md) — Rasterizes an analytical Sphere primitive into an SDF volume.
- [grid_to_index](grid_to_index.md) — Converts 3D grid indices (gx, gy, gz) into a 1D flat index.
- [grid_to_index](grid_to_index_1.md) — Converts 3D grid indices (gx, gy, gz) into a 1D flat index.
- [grid_to_world](grid_to_world.md) — Converts 3D grid coordinates to world-space coordinates (voxel center).
- [grid_to_world](grid_to_world_1.md) — Converts 3D grid coordinates to world-space coordinates (voxel center).
- [index_to_grid](index_to_grid.md) — Converts a 1D flat index into 3D grid indices (gx, gy, gz).
- [index_to_grid](index_to_grid_1.md) — Converts a 1D flat index into 3D grid indices (gx, gy, gz).
- [index_to_world](index_to_world.md) — Converts a 1D flat index directly to world-space coordinates.
- [index_to_world](index_to_world_1.md) — Converts a 1D flat index directly to world-space coordinates.
- [new_empty](new_empty.md) — Creates a new empty SDF volume with all voxels set to infinity (outside).
- [new_empty](new_empty_1.md) — Creates a new empty SDF volume with all voxels set to infinity (outside).
- [sd_polygon_2d](sd_polygon_2d.md) — Computes exact signed distance from point p to 2D polygon.
- [sd_polygon_2d](sd_polygon_2d_1.md) — Computes exact signed distance from point p to 2D polygon.
- [smooth_union](smooth_union.md) — Polynomial Smooth Boolean Union (fillet-like blending).
- [smooth_union](smooth_union_1.md) — Polynomial Smooth Boolean Union (fillet-like blending).
- [static_grid_to_world](static_grid_to_world.md) — Converts 3D grid coordinates to world-space coordinates (voxel center) statically.
- [static_grid_to_world](static_grid_to_world_1.md) — Converts 3D grid coordinates to world-space coordinates (voxel center) statically.
- [static_index_to_grid](static_index_to_grid.md) — Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self.
- [static_index_to_grid](static_index_to_grid_1.md) — Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self.
- [total_voxels](total_voxels.md) — Number of voxels in the grid.
- [total_voxels](total_voxels_1.md) — Number of voxels in the grid.
- [voxel_volume](voxel_volume.md) — Volume of a single voxel in mm³.
- [voxel_volume](voxel_volume_1.md) — Volume of a single voxel in mm³.
- [world_to_grid](world_to_grid.md) — Converts a world-space point to integer grid coordinates if within bounds.
- [world_to_grid](world_to_grid_1.md) — Converts a world-space point to integer grid coordinates if within bounds.
