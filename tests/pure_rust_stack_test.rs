//! Integration tests verifying all phases of the Pure-Rust Hermetic Single-Binary & Desktop Engine.

use audio_forge::{AudioCaptureRingBuffer, SpeechToTextEngine};
use edge_swarm::{DiscoveredNode, SwarmDiscoveryService, SwarmPairingManager, SwarmRole};
use mcp_probe_rs::AtomicFlashManager;
use media_forge::{DiffusionEngine, DiffusionRequest, SchedulerType};
use model_trainer::modelscope::{HubDownloader, HubSource};
use model_trainer::optimizer::{AdamW8bitOptimizer, AdamW8bitState, AdamWConfig};
use model_trainer::pure_rust_trainer::{PureRustTrainer, PureRustTrainerConfig};
use oxide_engines::universal_loader::{ContainerFormat, GGUF_MAGIC, UniversalModelContainer};
use oxide_kernels::ternary::TernaryBitplaneMatrix;
use qdrant_service::embedded_vector::VectorStore;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use surrealdb_service::embedded::EmbeddedSurrealDb;

#[test]
fn test_audio_capture_ring_buffer() {
    let ring = AudioCaptureRingBuffer::new(100);
    assert!(!ring.is_recording());

    ring.set_recording(true);
    assert!(ring.is_recording());

    let samples = vec![0.1f32, 0.2, 0.3, 0.4, 0.5];
    ring.push_samples(&samples);

    let recent = ring.get_recent_samples(3);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent, vec![0.3, 0.4, 0.5]);
}

#[tokio::test]
async fn test_stt_transcription_engine() {
    let engine = SpeechToTextEngine::new();
    let samples = vec![0.0f32; 1600];
    let res = engine.transcribe(&samples).await.unwrap();
    assert_eq!(res.language, "en");
    assert!(res.confidence > 0.9);
}

#[tokio::test]
async fn test_media_forge_diffusion_pipeline() {
    let engine = DiffusionEngine::new();
    let req = DiffusionRequest {
        prompt: "cyberpunk circuit board 3d render".to_string(),
        negative_prompt: None,
        model_id: "flux-schnell".to_string(),
        width: 64,
        height: 64,
        steps: 2,
        guidance_scale: 0.0,
        seed: Some(42),
        scheduler: SchedulerType::FlowMatchEuler,
    };

    let res = engine.generate(req).await.unwrap();
    assert_eq!(res.image_width, 64);
    assert_eq!(res.image_height, 64);
    assert_eq!(res.channels, 3);
    assert_eq!(res.raw_rgb.len(), 64 * 64 * 3);
    assert_eq!(res.seed_used, 42);
}

#[test]
fn test_modelscope_and_hf_hub_resolution() {
    let hf_url = HubDownloader::resolve_url(
        HubSource::HuggingFace,
        "Qwen/Qwen2.5-Coder-7B",
        "model.safetensors",
    );
    assert_eq!(
        hf_url,
        "https://huggingface.co/Qwen/Qwen2.5-Coder-7B/resolve/main/model.safetensors"
    );

    let ms_url = HubDownloader::resolve_url(
        HubSource::ModelScope,
        "qwen/Qwen2.5-Coder-7B",
        "model.safetensors",
    );
    assert_eq!(
        ms_url,
        "https://modelscope.cn/api/v1/models/qwen/Qwen2.5-Coder-7B/repo?Revision=master&FilePath=model.safetensors"
    );
}

#[tokio::test]
async fn test_edge_swarm_discovery_and_offload() {
    let local = DiscoveredNode {
        node_id: "laptop-edge".to_string(),
        role: SwarmRole::EdgeController,
        address: "192.168.1.101".to_string(),
        port: 4040,
        vram_capacity_mb: 4096,
        cpu_cores: 8,
        supported_modalities: vec!["text".to_string(), "audio".to_string()],
    };

    let discovery = SwarmDiscoveryService::new(local);
    let peers = discovery.discover_peers().await.unwrap();
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].role, SwarmRole::ComputeCore);

    let pairing = SwarmPairingManager::new("laptop-edge".to_string(), SwarmRole::EdgeController);
    let decision = pairing.evaluate_offload(200, false, &peers);
    assert!(decision.should_offload);
    assert_eq!(decision.target_node.unwrap().node_id, "desktop-titan");
}

#[test]
fn test_universal_model_loader_gguf_and_mmap() {
    let temp_path = std::env::temp_dir().join("test_stack_model.gguf");
    {
        let mut f = File::create(&temp_path).unwrap();
        f.write_all(&GGUF_MAGIC).unwrap();
        f.write_all(&3u32.to_le_bytes()).unwrap();
        f.write_all(&[0u8; 2048]).unwrap();
    }

    let container = UniversalModelContainer::load_file(&temp_path).unwrap();
    assert!(matches!(container.format, ContainerFormat::Gguf(3)));
    assert!(container.tensors.len() >= 2);
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn test_ternary_bitplane_xnor_gemm_kernel() {
    let weights = vec![1i8, 0, -1, 1, -1, 0, 1, 1];
    let mat = TernaryBitplaneMatrix::from_ternary_weights(&weights, 2, 4);
    let activations = vec![1.0f32, -1.0, 1.0, 1.0];
    let mut output = vec![0.0f32; 2];

    mat.xnor_gemm_cpu(&activations, &mut output);
    assert_eq!(output.len(), 2);
}

#[tokio::test]
async fn test_embedded_surrealdb_in_process() {
    let db = EmbeddedSurrealDb::in_memory().await.unwrap();
    let res = db
        .db
        .query("CREATE user:test_architect SET name = 'Faez', role = 'Lead Architect'")
        .await;
    assert!(res.is_ok());
}

#[test]
fn test_embedded_vector_store_retrieval() {
    let mut store = VectorStore::new();
    let mut payload = HashMap::new();
    payload.insert("chip".to_string(), serde_json::json!("STM32F407VG"));

    store
        .upsert("hal_specs", "doc_stm32", vec![1.0, 0.0, 0.0], payload)
        .unwrap();
    let results = store.search("hal_specs", &[0.95, 0.05, 0.0], 1);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "doc_stm32");
}

#[test]
fn test_pure_rust_trainer_and_8bit_adamw() {
    let config = PureRustTrainerConfig {
        max_steps: 5,
        warmup_steps: 1,
        ..Default::default()
    };
    let mut trainer = PureRustTrainer::new(config, 32);
    let dummy_data = vec![0.05f32; 32 * 2];

    let metrics = trainer.train_step(&dummy_data, 32, 1);
    assert_eq!(metrics.step, 1);
    assert!(metrics.loss > 0.0);

    let mut params = vec![2.0f32, -1.0f32];
    let mut state = AdamW8bitState::new(2, 64);
    let opt = AdamW8bitOptimizer::new(AdamWConfig {
        lr: 0.15,
        weight_decay: 0.0,
        ..Default::default()
    });

    for _ in 0..100 {
        let mut grads = params.clone();
        opt.step(&mut params, &mut grads, &mut state);
    }
    assert!(params[0].abs() < 0.5);
    assert!(params[1].abs() < 0.5);
}

#[test]
fn test_atomic_flash_hitl_token_validation() {
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
    let valid_token =
        AtomicFlashManager::sign_chip_authorization(&signing_key, "STM32F407VG", Some("flash_v1"));

    assert!(
        AtomicFlashManager::validate_ed25519_flash_token(&valid_token, "STM32F407VG").is_ok()
    );
    assert!(AtomicFlashManager::validate_ed25519_flash_token(&valid_token, "STM32F103").is_err());
    assert!(AtomicFlashManager::validate_ed25519_flash_token("", "STM32F407VG").is_err());
}

#[tokio::test]
async fn test_native_tool_registry_and_agentic_tool_dispatch() {
    use oxide_tooling::{NativeToolRegistry, OrnithPromptFormatter};

    let registry = NativeToolRegistry::with_defaults();
    let schemas = registry.export_schemas();
    assert_eq!(schemas.len(), 4);

    let tools = registry.list_tools();
    assert!(tools.contains(&"stair_search".to_string()));
    assert!(tools.contains(&"hardware_probe".to_string()));
    assert!(tools.contains(&"memory_recall".to_string()));
    assert!(tools.contains(&"ptx_decompile".to_string()));

    // Form Ornith prompt with registered tools
    let messages = vec![oxide_core::ChatMessage {
        role: oxide_core::Role::User,
        content: "Check connected hardware probes".into(),
        name: None,
    }];
    let formatted_prompt = OrnithPromptFormatter::format(&messages, Some(&schemas), None);
    assert!(formatted_prompt.contains("<tools>"));
    assert!(formatted_prompt.contains("hardware_probe"));

    // Simulate model output with XML tool call
    let simulated_model_completion = r#"I will query the attached hardware debug probes.
<tool_call>
{"name": "hardware_probe", "arguments": {"action": "list_probes"}}
</tool_call>"#;

    let calls = OrnithPromptFormatter::extract_tool_calls(simulated_model_completion);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "hardware_probe");

    // Execute in-process native tool dispatch
    let result = registry.dispatch(&calls[0]).await.unwrap();
    assert_eq!(result["action"], "list_probes");
    assert_eq!(result["count"], 1);
}

#[test]
fn test_horizon3_closed_loop_eda_and_smt_circuit_safety() {
    use circuit_forge::{CircuitBuilder, run_erc};
    use formal_verify::smt_solver::{CircuitState, verify_circuit_safety_invariants};

    let mut builder = CircuitBuilder::new();
    builder.add_component("U1", "MCU_ST_STM32F401:STM32F401RETx", "STM32F401");
    builder.add_component("C1", "Device:C_Small", "100nF");
    builder.connect_net("U1", "1", "+3.3V").unwrap();
    builder.connect_net("U1", "64", "GND").unwrap();
    builder.connect_net("C1", "1", "+3.3V").unwrap();
    builder.connect_net("C1", "2", "GND").unwrap();

    let graph = builder.build();
    let erc_report = run_erc(&graph);
    assert!(erc_report.errors.is_empty());

    let states = vec![
        CircuitState {
            state_id: "Normal_Idle".to_string(),
            voltage: 3.3,
            max_voltage: 3.6,
            current: 0.045,
            max_current: 0.150,
            has_fault: false,
            is_isolated: false,
        },
        CircuitState {
            state_id: "PLL_Active_84MHz".to_string(),
            voltage: 3.3,
            max_voltage: 3.6,
            current: 0.082,
            max_current: 0.150,
            has_fault: false,
            is_isolated: false,
        },
    ];

    let proof = verify_circuit_safety_invariants(&states);
    assert!(proof.is_safe, "SMT-LIB2 invariant proof failed");
    assert!(proof.violated_states.is_empty());
    assert!(proof.smt_lib2_formula.contains("(set-logic QF_LRA)"));
}

#[test]
fn test_horizon4_mechanical_cad_brep_and_voxelization() {
    use cad_forge::dsl::CadBuilder;
    use cad_forge::voxelizer::VoxelGrid;
    use glam::Vec3;

    let builder = CadBuilder::new();
    let (builder, box_idx) = builder.add_box(Vec3::ZERO, Vec3::new(40.0, 40.0, 10.0));
    let (builder, hole_idx) = builder.add_cylinder(Vec3::ZERO, 2.5, 12.0);
    let script = builder.subtract(box_idx, hole_idx).build();

    assert_eq!(script.primitives.len(), 2);
    assert_eq!(script.operations.len(), 1);

    let voxels = VoxelGrid::from_box(Vec3::ZERO, Vec3::new(40.0, 40.0, 10.0), 1.0);
    assert!(
        !voxels.occupied.is_empty(),
        "Solid must produce active voxels"
    );
}

#[test]
fn test_horizon5_sovereign_quic_mesh_wire_packet_security() {
    use oxide_network::wire::{PROTOCOL_MAGIC, PreParseVerdict, ReplayWindow128, pre_parse_packet};

    let mut window = ReplayWindow128::new();
    assert!(window.check_and_update(1));
    assert!(window.check_and_update(2));
    assert!(window.check_and_update(10));
    assert!(window.check_and_update(5));
    assert!(!window.check_and_update(5));
    assert!(!window.check_and_update(1));

    // 2. Anti-DPI junk frame classification and malformed packet rejection
    let junk = vec![0xDE, 0xAD, 0xBE, 0xEF];
    assert_eq!(pre_parse_packet(&junk), PreParseVerdict::JunkIgnored);

    let malformed = vec![0x01, 0x02, 0x03, 0x04];
    assert_eq!(pre_parse_packet(&malformed), PreParseVerdict::Malformed);

    // 3. Header verification with PROTOCOL_MAGIC (16 bytes aligned)
    let mut valid_header = vec![0u8; 16];
    valid_header[0..4].copy_from_slice(&PROTOCOL_MAGIC.to_le_bytes());
    valid_header[6] = 0x10;
    assert_eq!(
        pre_parse_packet(&valid_header),
        PreParseVerdict::ValidControl {
            packet_id: 0,
            payload_len: 0
        }
    );
}

#[tokio::test]
async fn test_horizon6_self_evolution_wasm_tool_registry() {
    use self_evolver::skill_crystallizer::CrystallizedSkill;
    use self_evolver::tool_maker::WasmToolRegistry;

    let mut registry = WasmToolRegistry::new();
    let dummy_wasm_bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
    registry.register_tool("synthesized_crc32_calc", dummy_wasm_bytes.clone());

    let registered = registry.get_tool("synthesized_crc32_calc");
    assert!(registered.is_some());
    assert_eq!(registered.unwrap(), &dummy_wasm_bytes);

    let skill = CrystallizedSkill {
        name: "stm32-swd-flasher".to_string(),
        description: "Automated firmware flash and watchdog verification for STM32".to_string(),
        tags: vec![
            "embedded".to_string(),
            "stm32".to_string(),
            "swd".to_string(),
        ],
        prompt_template: "Flash firmware {{binary}} to {{chip}}".to_string(),
        step_sequence: vec![
            "Probe target SWD connection".to_string(),
            "Erase sector 0 and program binary".to_string(),
            "Verify hardware watchdog refresh".to_string(),
        ],
        source_task_id: "task_01a".to_string(),
    };

    let markdown = skill.to_markdown();
    assert!(markdown.contains("name: stm32-swd-flasher"));
    assert!(markdown.contains("## Execution Workflow"));
    assert!(markdown.contains("1. Probe target SWD connection"));
}

#[tokio::test]
async fn test_horizon7_heterogeneous_cross_domain_co_simulation() {
    use cross_domain_verifier::CrossDomainVerifier;
    use oxide_protocol::DtxId;

    let verifier = CrossDomainVerifier::new(85.0, 1.5);
    let dtx_id = DtxId::new_v7();

    let report = verifier
        .run_co_simulation(dtx_id, 1.0, 0.45, "Aluminum_6061")
        .await
        .expect("Co-simulation must converge safely");

    assert!(report.passed, "Verification report must pass");
    assert!(
        report.peak_temperature_c < 85.0,
        "Silicon temp must not exceed 85C"
    );
    assert!(
        report.residual < 1e-3,
        "Co-simulation residual must converge"
    );
    assert!(
        report.iterations < 20,
        "Fixed point iteration must converge within max iterations"
    );
}
