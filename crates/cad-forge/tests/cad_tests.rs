use cad_forge::{CadBuilder, VoxelGrid, compute_iou};
use cad_forge::verifier::generate_diff_map;
use glam::Vec3;

#[test]
fn test_cad_builder_fluent_api() {
    let (builder, b_idx) = CadBuilder::new().add_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0));
    let (builder, c_idx) = builder.add_cylinder(Vec3::ZERO, 2.0, 12.0);
    let script = builder.subtract(b_idx, c_idx).build();

    assert_eq!(script.primitives.len(), 2);
    assert_eq!(script.operations.len(), 1);
}

#[test]
fn test_voxel_grid_identical_box_iou() {
    let grid_a = VoxelGrid::from_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0), 1.0);
    let grid_b = VoxelGrid::from_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0), 1.0);

    let iou = compute_iou(&grid_a, &grid_b);
    assert!((iou - 1.0).abs() < 1e-6);
}

#[test]
fn test_voxel_grid_disjoint_boxes_iou() {
    let grid_a = VoxelGrid::from_box(Vec3::new(0.0, 0.0, 0.0), Vec3::new(5.0, 5.0, 5.0), 1.0);
    let grid_b = VoxelGrid::from_box(Vec3::new(50.0, 50.0, 50.0), Vec3::new(5.0, 5.0, 5.0), 1.0);

    let iou = compute_iou(&grid_a, &grid_b);
    assert_eq!(iou, 0.0);
}

#[test]
fn test_csg_subtraction_voxelization() {
    // Solid box
    let solid_box = VoxelGrid::from_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0), 1.0);
    let solid_count = solid_box.occupied.len();

    // Box with drilled hole
    let (builder, b_idx) = CadBuilder::new().add_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0));
    let (builder, c_idx) = builder.add_cylinder(Vec3::ZERO, 2.0, 12.0);
    let script = builder.subtract(b_idx, c_idx).build();

    let drilled_box = VoxelGrid::from_script(&script, 1.0);
    let drilled_count = drilled_box.occupied.len();

    // Volume of drilled box must be strictly less than solid box
    assert!(drilled_count < solid_count);

    let diff = generate_diff_map(&drilled_box, &solid_box);
    assert_eq!(diff.false_positive_count, 0); // Drilled box contains no voxels outside the solid box
    assert!(diff.false_negative_count > 0);   // The drilled hole voxels are the difference
    assert!(diff.iou > 0.7 && diff.iou < 1.0);
}
