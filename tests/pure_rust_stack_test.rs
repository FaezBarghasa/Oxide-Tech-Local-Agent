//! Integration tests verifying all phases of the Pure-Rust Hermetic Single-Binary & Desktop Engine.

use audio_forge::{AudioCaptureRingBuffer, SpeechToTextEngine};
use edge_swarm::{DiscoveredNode, SwarmDiscoveryService, SwarmPairingManager, SwarmRole};
use media_forge::{DiffusionEngine, DiffusionRequest, SchedulerType};
use model_trainer::modelscope::{HubDownloader, HubSource};

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
