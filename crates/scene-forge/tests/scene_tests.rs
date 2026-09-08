use uuid::Uuid;

use scene_forge::{
    BlenderBridge, BlenderCommand, BlenderResponse, GeometryVerifier, InteractionMode,
    MacroCodegen, MockBlenderEngine, PrimitiveType, ShadowSceneGraph, primitives,
};

#[test]
fn test_shadow_scene_graph_context_auto_correction() {
    let mut shadow = ShadowSceneGraph::new();
    let cube_id = Uuid::new_v4();
    shadow.register_object(cube_id, "SciFiCube", [2.0, 2.0, 2.0], None);

    // Initial state: active_object = None, mode = Object
    assert_eq!(shadow.active_object, None);
    assert_eq!(shadow.mode, InteractionMode::Object);

    // 1. Prepare context for an extrusion on the cube in Edit Mode (Faces)
    let edit_faces = InteractionMode::edit_faces();
    let auto_cmds = shadow.prepare_context(edit_faces, cube_id);

    // Verify auto-injected synchronization commands
    assert_eq!(auto_cmds.len(), 2);
    assert_eq!(auto_cmds[0], BlenderCommand::SetActiveObject(cube_id));
    assert_eq!(auto_cmds[1], BlenderCommand::SwitchMode(edit_faces));

    // Shadow graph internal state is now updated
    assert_eq!(shadow.active_object, Some(cube_id));
    assert_eq!(shadow.mode, edit_faces);

    // 2. Prepare context again with the same target and mode -> 0 commands needed
    let repeat_cmds = shadow.prepare_context(edit_faces, cube_id);
    assert!(
        repeat_cmds.is_empty(),
        "Expected no redundant state commands"
    );
}

#[tokio::test]
async fn test_postcard_binary_ipc_roundtrip() {
    let (client_stream, mut server_stream) = tokio::io::duplex(4096);
    let mut bridge = BlenderBridge::new(client_stream);

    // Spawn mock server task handling length-prefixed postcard packets
    let server_task = tokio::spawn(async move {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut engine = MockBlenderEngine::new();

        // Read 4-byte length prefix
        let mut len_buf = [0u8; 4];
        server_stream.read_exact(&mut len_buf).await.unwrap();
        let cmd_len = u32::from_le_bytes(len_buf) as usize;

        // Read payload
        let mut payload = vec![0u8; cmd_len];
        server_stream.read_exact(&mut payload).await.unwrap();

        // Deserialize command
        let cmd: BlenderCommand = postcard::from_bytes(&payload).unwrap();
        let response = engine.handle_command(cmd);

        // Send length-prefixed response
        let resp_payload = postcard::to_allocvec(&response).unwrap();
        let resp_len = (resp_payload.len() as u32).to_le_bytes();
        server_stream.write_all(&resp_len).await.unwrap();
        server_stream.write_all(&resp_payload).await.unwrap();
        server_stream.flush().await.unwrap();
    });

    // Client executes command
    let cmd = BlenderCommand::CreatePrimitive {
        primitive_type: PrimitiveType::Cube,
        dimensions: [1.0, 1.0, 1.0],
    };

    let res = bridge.execute(cmd).await.expect("IPC roundtrip failed");
    match res {
        BlenderResponse::ObjectCreated(id) => {
            assert!(!id.is_nil());
        }
        other => panic!("Expected ObjectCreated, got: {:?}", other),
    }

    server_task.await.unwrap();
}

#[test]
fn test_manifold_verification_watertight_cube() {
    let cube_mesh = primitives::cube([2.0, 2.0, 2.0]);
    let verifier = GeometryVerifier::new();

    let report = verifier.assert_manifold(&cube_mesh);

    assert!(report.is_manifold, "Cube should have no non-manifold edges");
    assert!(report.is_closed, "Cube should be a watertight closed solid");
    assert_eq!(
        report.boundary_edges.len(),
        0,
        "Closed solid should have zero boundary edges"
    );
    assert_eq!(
        report.non_manifold_edges.len(),
        0,
        "Closed solid should have zero non-manifold edges"
    );
    assert_eq!(
        report.euler_characteristic, 2,
        "Euler characteristic V - E + F for genus-0 sphere/cube must be 2"
    );
    assert!(report.is_valid_solid());
    assert!(verifier.generate_error_map(&report).is_none());
}

#[test]
fn test_non_manifold_detection_missing_face() {
    let mut broken_cube = primitives::cube([2.0, 2.0, 2.0]);
    // Remove one triangle face to create a hole
    broken_cube.indices.pop();

    let verifier = GeometryVerifier::new();
    let report = verifier.assert_manifold(&broken_cube);

    assert!(!report.is_closed, "Mesh with missing face cannot be closed");
    assert!(
        !report.boundary_edges.is_empty(),
        "Hole must produce boundary edges"
    );
    assert!(!report.is_valid_solid());

    let error_map = verifier
        .generate_error_map(&report)
        .expect("Expected error map");
    assert_eq!(error_map.rule, "TOPOLOGY_OPEN_BOUNDARY");
    assert_eq!(error_map.severity, "ERROR");
    assert!(error_map.message.contains("boundary edges detected"));
}

#[test]
fn test_signed_volume_calculation() {
    let dimensions = [2.0, 3.0, 4.0];
    let cube_mesh = primitives::cube(dimensions);
    let verifier = GeometryVerifier::new();

    let analytical_volume = 2.0 * 3.0 * 4.0; // 24.0
    let calculated_volume = verifier.calculate_signed_volume(&cube_mesh);

    assert!(
        (calculated_volume - analytical_volume).abs() < 1e-4,
        "Calculated signed volume {} did not match analytical volume {}",
        calculated_volume,
        analytical_volume
    );

    assert!(verifier.assert_volume(&cube_mesh, 24.0, 1e-4));
}

#[test]
fn test_macro_codegen_synthesis() {
    let cube_id = Uuid::new_v4();
    let commands = vec![
        BlenderCommand::SetActiveObject(cube_id),
        BlenderCommand::SwitchMode(InteractionMode::edit_faces()),
        BlenderCommand::ExtrudeSelection {
            axis: [0.0, 1.0, 0.0],
            distance: 0.5,
        },
        BlenderCommand::BevelSelection {
            offset: 0.05,
            segments: 2,
        },
    ];

    let macro_code = MacroCodegen::generate_macro("sci_fi_crate", &commands);
    assert!(macro_code.contains("macro_rules! sci_fi_crate"));
    assert!(macro_code.contains("BlenderCommand::SetActiveObject"));
    assert!(macro_code.contains("BlenderCommand::SwitchMode"));
    assert!(macro_code.contains("BlenderCommand::ExtrudeSelection"));
    assert!(macro_code.contains("BlenderCommand::BevelSelection"));
}
