use glam::Vec3;
use parametric_forge::{CadOperation, Plane};
use uuid::Uuid;
use visual_forge::{
    GeometryCorrectionReport, KernelError, SDFVolume, ShadowKernel, VisualFeedbackOptimizer,
    VisualMatchCodegen,
};

#[test]
fn test_sdf_volume_primitives_and_csg() {
    let bounds = [Vec3::new(-20.0, -20.0, -20.0), Vec3::new(20.0, 20.0, 20.0)];
    let resolution = 1.0f32;

    // 1. Box primitive
    let box_vol = SDFVolume::from_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0), bounds, resolution);
    // Center should be inside (< 0.0)
    let center_grid = box_vol.world_to_grid(Vec3::ZERO).unwrap();
    let center_idx = box_vol.grid_to_index(center_grid[0], center_grid[1], center_grid[2]);
    assert!(box_vol.voxels[center_idx] < 0.0);

    // Far point should be outside (> 0.0)
    let outside_grid = box_vol.world_to_grid(Vec3::new(15.0, 15.0, 15.0)).unwrap();
    let outside_idx = box_vol.grid_to_index(outside_grid[0], outside_grid[1], outside_grid[2]);
    assert!(box_vol.voxels[outside_idx] > 0.0);

    // 2. Sphere primitive
    let sphere_vol = SDFVolume::from_sphere(Vec3::ZERO, 5.0, bounds, resolution);
    assert!(sphere_vol.voxels[center_idx] < 0.0);

    // 3. Cylinder primitive
    let cyl_vol = SDFVolume::from_cylinder(Vec3::ZERO, 3.0, 12.0, bounds, resolution);
    assert!(cyl_vol.voxels[center_idx] < 0.0);

    // 4. CSG Boolean Difference (Box - Cylinder = Box with central hole)
    let hollow_box = box_vol.boolean_difference(&cyl_vol);
    // Center was inside cylinder, so now in difference it should be OUTSIDE/EMPTY (> 0.0)
    assert!(hollow_box.voxels[center_idx] >= 0.0);

    // 5. CSG Boolean Union
    let union_vol = box_vol.boolean_union(&sphere_vol);
    assert!(union_vol.voxels[center_idx] < 0.0);

    // 6. Smooth Boolean Union
    let smooth_union = box_vol.smooth_union(&sphere_vol, 2.0);
    assert!(smooth_union.voxels[center_idx] < 0.0);
}

#[test]
fn test_volumetric_diff_and_error_report() {
    let bounds = [Vec3::new(-15.0, -15.0, -15.0), Vec3::new(15.0, 15.0, 15.0)];
    let resolution = 1.0f32;

    // Target: Box with cylindrical hole
    let target_box =
        SDFVolume::from_box(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0), bounds, resolution);
    let target_cyl = SDFVolume::from_cylinder(Vec3::ZERO, 2.0, 12.0, bounds, resolution);
    let target = target_box.boolean_difference(&target_cyl);

    // Generated: Solid Box (missing the hole -> excess material in the hole)
    let generated = target_box.clone();

    let report: GeometryCorrectionReport = generated.compute_volumetric_diff(&target);

    assert!(report.excess_material_mm3 > 0.0);
    assert!(report.missing_material_mm3 == 0.0);
    assert!(report.volumetric_iou < 1.0);
    assert!(report.volumetric_iou > 0.80);
    assert!(report.correction_hint.contains("Excess material"));
    assert!(report.correction_hint.contains("Boolean Difference"));
}

#[test]
fn test_shadow_kernel_safe_evaluation() {
    let mut kernel = ShadowKernel::new([-30.0, -30.0, -30.0], [30.0, 30.0, 30.0], 1.0);

    // Valid Sketch
    let sketch_id = Uuid::new_v4();
    let sketch_op = CadOperation::Sketch2D {
        id: sketch_id,
        name: "base_sketch".to_string(),
        plane: Plane::XY,
        points: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
        constraints: vec![],
        entities: vec![],
    };
    let rep1 = kernel
        .evaluate_operation(&sketch_op)
        .expect("Valid sketch should succeed");
    assert_eq!(rep1.id, sketch_id);

    // Valid Extrude
    let extrude_id = Uuid::new_v4();
    let extrude_op = CadOperation::Extrude {
        id: extrude_id,
        name: "base_extrude".to_string(),
        profile_id: sketch_id,
        distance: 8.0,
        direction: [0.0, 0.0, 1.0],
    };
    let rep2 = kernel
        .evaluate_operation(&extrude_op)
        .expect("Valid extrude should succeed");
    assert_eq!(rep2.id, extrude_id);
    assert!(rep2.estimated_volume_mm3 > 0.0);

    // Invalid Extrude: negative distance
    let invalid_extrude_id = Uuid::new_v4();
    let invalid_extrude_op = CadOperation::Extrude {
        id: invalid_extrude_id,
        name: "bad_extrude".to_string(),
        profile_id: sketch_id,
        distance: -5.0,
        direction: [0.0, 0.0, 1.0],
    };
    let err = kernel.evaluate_operation(&invalid_extrude_op).unwrap_err();
    assert!(matches!(err, KernelError::OutOfBoundsFeature(_)));

    // Invalid Sketch: collinear points
    let bad_sketch_id = Uuid::new_v4();
    let bad_sketch_op = CadOperation::Sketch2D {
        id: bad_sketch_id,
        name: "collinear_sketch".to_string(),
        plane: Plane::XY,
        points: vec![[0.0, 0.0], [5.0, 0.0], [10.0, 0.0]],
        constraints: vec![],
        entities: vec![],
    };
    let err2 = kernel.evaluate_operation(&bad_sketch_op).unwrap_err();
    assert!(matches!(err2, KernelError::DegenerateFeature(_)));
}

#[test]
fn test_closed_loop_feedback_optimization() {
    let bounds = [Vec3::new(-10.0, -10.0, -10.0), Vec3::new(10.0, 10.0, 10.0)];
    let resolution = 1.0f32;

    // Target: Box 8x8x8
    let target_sdf = SDFVolume::from_box(Vec3::ZERO, Vec3::new(8.0, 8.0, 8.0), bounds, resolution);

    let mut optimizer = VisualFeedbackOptimizer::new(target_sdf.clone(), 0.98, 1.0);

    // Iteration 1: Undersized box 4x4x4 (low IoU)
    let step1_sdf = SDFVolume::from_box(Vec3::ZERO, Vec3::new(4.0, 4.0, 4.0), bounds, resolution);
    let rep1 = optimizer.evaluate_step(&step1_sdf);
    assert!(!optimizer.is_converged());
    assert!(rep1.missing_material_mm3 > 0.0);

    // Iteration 2: Closer box 7x7x7
    let step2_sdf = SDFVolume::from_box(Vec3::ZERO, Vec3::new(7.0, 7.0, 7.0), bounds, resolution);
    let _rep2 = optimizer.evaluate_step(&step2_sdf);
    let (iou2, is_improving) = optimizer.convergence_trend();
    assert!(is_improving);
    assert!(iou2 > rep1.volumetric_iou);

    // Iteration 3: Exact match 8x8x8
    let step3_sdf = target_sdf.clone();
    let rep3 = optimizer.evaluate_step(&step3_sdf);
    assert!(optimizer.is_converged());
    assert!(rep3.volumetric_iou >= 0.999);
}

#[test]
fn test_visual_match_macro_codegen() {
    let code = VisualMatchCodegen::generate_macro("mounting_bracket", 0.05, 0.9985, 3)
        .expect("Macro codegen should succeed");

    assert!(code.contains("nexus_macro::visual_match!"));
    assert!(code.contains("target: \"mounting_bracket\""));
    assert!(code.contains("volumetric_iou: 0.9985"));
    assert!(code.contains("CONVERGED_VERIFIED"));
}
