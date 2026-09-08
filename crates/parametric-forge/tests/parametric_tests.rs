use uuid::Uuid;

use parametric_forge::{
    CadOperation, ConstraintSolver, DagEvaluator, DeltaPatch, FeatureDAG, GeometricConstraint,
    HierarchyLevel, MacroCodegen, Mutation, Plane, SketchEntity,
};

#[test]
fn test_2d_geometric_constraint_solver() {
    // 4 points of a rectangle with initial guesses
    let mut points = vec![
        [0.0, 0.0], // p0
        [9.5, 0.2], // p1 (guess near 10, 0)
        [9.8, 4.8], // p2 (guess near 10, 5)
        [0.1, 5.2], // p3 (guess near 0, 5)
    ];

    let constraints = vec![
        // 1. Fix p0 at (0, 0)
        GeometricConstraint::FixPoint {
            point: 0,
            x: 0.0,
            y: 0.0,
        },
        // 2. p0->p1 is horizontal
        GeometricConstraint::Horizontal { p1: 0, p2: 1 },
        // 3. Distance p0->p1 is 10.0
        GeometricConstraint::Distance {
            p1: 0,
            p2: 1,
            distance: 10.0,
        },
        // 4. p1->p2 is vertical
        GeometricConstraint::Vertical { p1: 1, p2: 2 },
        // 5. Distance p1->p2 is 5.0
        GeometricConstraint::Distance {
            p1: 1,
            p2: 2,
            distance: 5.0,
        },
        // 6. p2->p3 is horizontal
        GeometricConstraint::Horizontal { p1: 2, p2: 3 },
        // 7. p3->p0 is vertical
        GeometricConstraint::Vertical { p1: 3, p2: 0 },
        // 8. Distance p2->p3 is 10.0
        GeometricConstraint::Distance {
            p1: 2,
            p2: 3,
            distance: 10.0,
        },
    ];

    ConstraintSolver::solve_sketch(&constraints, &mut points, 100, 1e-6)
        .expect("Constraint solver should converge");

    let p0 = points[0];
    let p1 = points[1];
    let p2 = points[2];
    let p3 = points[3];

    assert!((p0[0] - 0.0).abs() < 1e-4);
    assert!((p0[1] - 0.0).abs() < 1e-4);

    assert!((p1[0] - 10.0).abs() < 1e-4);
    assert!((p1[1] - 0.0).abs() < 1e-4);

    assert!((p2[0] - 10.0).abs() < 1e-4);
    assert!((p2[1] - 5.0).abs() < 1e-4);

    assert!((p3[0] - 0.0).abs() < 1e-4);
    assert!((p3[1] - 5.0).abs() < 1e-4);
}

#[test]
fn test_feature_dag_construction_and_topological_sort() {
    let mut dag = FeatureDAG::new();

    // 1. Sketch
    let sketch_id = Uuid::new_v4();
    let points = vec![[0.0, 0.0], [20.0, 0.0], [20.0, 10.0], [0.0, 10.0]];
    let constraints = vec![
        GeometricConstraint::FixPoint {
            point: 0,
            x: 0.0,
            y: 0.0,
        },
        GeometricConstraint::Horizontal { p1: 0, p2: 1 },
        GeometricConstraint::Distance {
            p1: 0,
            p2: 1,
            distance: 20.0,
        },
        GeometricConstraint::Vertical { p1: 1, p2: 2 },
        GeometricConstraint::Distance {
            p1: 1,
            p2: 2,
            distance: 10.0,
        },
        GeometricConstraint::Horizontal { p1: 2, p2: 3 },
        GeometricConstraint::Vertical { p1: 3, p2: 0 },
    ];
    let entities = vec![
        SketchEntity::Line { p1: 0, p2: 1 },
        SketchEntity::Line { p1: 1, p2: 2 },
        SketchEntity::Line { p1: 2, p2: 3 },
        SketchEntity::Line { p1: 3, p2: 0 },
    ];

    dag.add_operation(CadOperation::Sketch2D {
        id: sketch_id,
        name: "sketch_base".to_string(),
        plane: Plane::XY,
        points,
        constraints,
        entities,
    });

    // 2. Extrude
    let extrude_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Extrude {
        id: extrude_id,
        name: "extrude_base".to_string(),
        profile_id: sketch_id,
        distance: 15.0,
        direction: [0.0, 0.0, 1.0],
    });

    // 3. Fillet
    let fillet_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Fillet {
        id: fillet_id,
        name: "fillet_top".to_string(),
        target_op_id: extrude_id,
        target_edges: vec![Uuid::new_v4()],
        radius: 2.5,
    });

    let order = dag
        .topological_order()
        .expect("DAG should have valid topological ordering");
    assert_eq!(order.len(), 3);

    let summary = dag.semantic_summary();
    assert!(summary.contains("sketch_base"));
    assert!(summary.contains("extrude_base"));
    assert!(summary.contains("fillet_top"));
}

#[test]
fn test_hierarchy_aware_delta_patching() {
    let mut dag = FeatureDAG::new();

    let sketch_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Sketch2D {
        id: sketch_id,
        name: "sketch_mount".to_string(),
        plane: Plane::XY,
        points: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]],
        constraints: vec![],
        entities: vec![],
    });

    let extrude_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Extrude {
        id: extrude_id,
        name: "extrude_mount".to_string(),
        profile_id: sketch_id,
        distance: 10.0,
        direction: [0.0, 0.0, 1.0],
    });

    let fillet_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Fillet {
        id: fillet_id,
        name: "fillet_mount".to_string(),
        target_op_id: extrude_id,
        target_edges: vec![Uuid::new_v4()],
        radius: 1.0,
    });

    // Apply Extrusion mutation
    let patch1 = DeltaPatch::new(
        extrude_id,
        HierarchyLevel::Extrusion,
        Mutation::UpdateExtrusionDepth(25.0),
    );
    dag.apply_patch(&patch1)
        .expect("Applying extrusion patch should succeed");

    // Apply Fillet mutation
    let patch2 = DeltaPatch::new(
        fillet_id,
        HierarchyLevel::Fillet,
        Mutation::UpdateFilletRadius(2.0),
    );
    dag.apply_patch(&patch2)
        .expect("Applying fillet patch should succeed");

    // Verify DAG operations were mutated in-place
    let summary = dag.semantic_summary();
    assert!(summary.contains("25.00mm"));
    assert!(summary.contains("2.00mm"));
}

#[test]
fn test_dag_evaluation_and_geometric_validation() {
    let mut dag = FeatureDAG::new();

    let sketch_id = Uuid::new_v4();
    let points = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let constraints = vec![
        GeometricConstraint::FixPoint {
            point: 0,
            x: 0.0,
            y: 0.0,
        },
        GeometricConstraint::Horizontal { p1: 0, p2: 1 },
        GeometricConstraint::Distance {
            p1: 0,
            p2: 1,
            distance: 10.0,
        },
        GeometricConstraint::Vertical { p1: 1, p2: 2 },
        GeometricConstraint::Distance {
            p1: 1,
            p2: 2,
            distance: 10.0,
        },
        GeometricConstraint::Horizontal { p1: 2, p2: 3 },
        GeometricConstraint::Vertical { p1: 3, p2: 0 },
    ];
    let entities = vec![
        SketchEntity::Line { p1: 0, p2: 1 },
        SketchEntity::Line { p1: 1, p2: 2 },
        SketchEntity::Line { p1: 2, p2: 3 },
        SketchEntity::Line { p1: 3, p2: 0 },
    ];
    dag.add_operation(CadOperation::Sketch2D {
        id: sketch_id,
        name: "sketch_box".to_string(),
        plane: Plane::XY,
        points,
        constraints,
        entities,
    });

    let extrude_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Extrude {
        id: extrude_id,
        name: "extrude_box".to_string(),
        profile_id: sketch_id,
        distance: 10.0,
        direction: [0.0, 0.0, 1.0],
    });

    let evaluator = DagEvaluator::new();
    let solids = evaluator
        .evaluate(&mut dag)
        .expect("DAG evaluation should succeed");
    assert_eq!(solids.len(), 1);

    let solid = &solids[0];
    assert_eq!(solid.name, "extrude_box");
    assert!((solid.volume - 1000.0).abs() < 1e-3);
    assert!((solid.bounding_box.0[0] - 0.0).abs() < 1e-3);
    assert!((solid.bounding_box.1[0] - 10.0).abs() < 1e-3);
    assert!((solid.bounding_box.1[2] - 10.0).abs() < 1e-3);
}

#[test]
fn test_macro_codegen_synthesis() {
    let mut dag = FeatureDAG::new();
    let sketch_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Sketch2D {
        id: sketch_id,
        name: "s1".to_string(),
        plane: Plane::XY,
        points: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 5.0]],
        constraints: vec![],
        entities: vec![],
    });
    let extrude_id = Uuid::new_v4();
    dag.add_operation(CadOperation::Extrude {
        id: extrude_id,
        name: "e1".to_string(),
        profile_id: sketch_id,
        distance: 12.0,
        direction: [0.0, 0.0, 1.0],
    });

    let patches = vec![DeltaPatch::new(
        extrude_id,
        HierarchyLevel::Extrusion,
        Mutation::UpdateExtrusionDepth(20.0),
    )];

    let code =
        MacroCodegen::generate_macro(&dag, &patches).expect("Macro generation should succeed");
    assert!(code.contains("nexus_macro::parametric_edit!"));
    assert!(code.contains("sketch(id:"));
    assert!(code.contains("extrude(id:"));
    assert!(code.contains("UpdateExtrusionDepth(20.0)"));
}
