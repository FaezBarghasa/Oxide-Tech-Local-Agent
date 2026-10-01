//! Integration tests verifying all phases of the Pure-Rust Hermetic Single-Binary & Desktop Engine.

use audio_forge::{AudioCaptureRingBuffer, SpeechToTextEngine};
use edge_swarm::{DiscoveredNode, SwarmDiscoveryService, SwarmPairingManager, SwarmRole};
use mcp_probe_rs::AtomicFlashManager;
use media_forge::{DiffusionEngine, DiffusionRequest, SchedulerType};
use model_trainer::modelscope::{HubDownloader, HubSource};
use model_trainer::optimizer::{AdamW8bitOptimizer, AdamW8bitState, AdamWConfig};
use model_trainer::pure_rust_trainer::{PureRustTrainer, PureRustTrainerConfig};
use oxide_engines::universal_loader::{ContainerFormat, UniversalModelContainer, GGUF_MAGIC};
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
    let hf_url = HubDownloader::resolve_url(HubSource::HuggingFace, "Qwen/Qwen2.5-Coder-7B", "model.safetensors");
    assert_eq!(hf_url, "https://huggingface.co/Qwen/Qwen2.5-Coder-7B/resolve/main/model.safetensors");

    let ms_url = HubDownloader::resolve_url(HubSource::ModelScope, "qwen/Qwen2.5-Coder-7B", "model.safetensors");
    assert_eq!(ms_url, "https://modelscope.cn/api/v1/models/qwen/Qwen2.5-Coder-7B/repo?Revision=master&FilePath=model.safetensors");
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
    let res = db.db.query("CREATE user:test_architect SET name = 'Faez', role = 'Lead Architect'").await;
    assert!(res.is_ok());
}

#[test]
fn test_embedded_vector_store_retrieval() {
    let mut store = VectorStore::new();
    let mut payload = HashMap::new();
    payload.insert("chip".to_string(), serde_json::json!("STM32F407VG"));

    store.upsert("hal_specs", "doc_stm32", vec![1.0, 0.0, 0.0], payload).unwrap();
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
    assert!(AtomicFlashManager::validate_ed25519_flash_token("ed25519-sig-auth-stm32f407-valid-tok", "STM32F407VG").is_ok());
    assert!(AtomicFlashManager::validate_ed25519_flash_token("", "STM32F407VG").is_err());
}
