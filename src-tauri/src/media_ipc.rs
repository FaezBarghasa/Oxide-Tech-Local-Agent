use media_forge::{DiffusionEngine, DiffusionRequest, DiffusionResponse, SchedulerType, VideoEngine, VideoGenerationRequest, VideoGenerationResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

static MEDIA_STATE: std::sync::LazyLock<Arc<Mutex<MediaState>>> = std::sync::LazyLock::new(|| {
    Arc::new(Mutex::new(MediaState::new()))
});

struct MediaState {
    diffusion: DiffusionEngine,
    video: VideoEngine,
}

impl MediaState {
    fn new() -> Self {
        Self {
            diffusion: DiffusionEngine::new(),
            video: VideoEngine::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateImageDto {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub steps: Option<u32>,
    pub guidance_scale: Option<f32>,
    pub seed: Option<u64>,
    pub scheduler: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateVideoDto {
    pub prompt: String,
    pub num_frames: Option<u32>,
    pub fps: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[tauri::command]
pub async fn media_generate_image(req: GenerateImageDto) -> Result<DiffusionResponse, String> {
    let state = MEDIA_STATE.lock().await;
    let scheduler = match req.scheduler.as_deref() {
        Some("EulerDiscrete") => SchedulerType::EulerDiscrete,
        Some("EulerAncestral") => SchedulerType::EulerAncestral,
        Some("DDIM") => SchedulerType::DDIM,
        Some("DPMSolverPlusPlus") => SchedulerType::DPMSolverPlusPlus,
        _ => SchedulerType::FlowMatchEuler,
    };

    let diff_req = DiffusionRequest {
        prompt: req.prompt.clone(),
        negative_prompt: req.negative_prompt,
        model_id: "flux-schnell".to_string(),
        width: req.width.unwrap_or(512),
        height: req.height.unwrap_or(512),
        steps: req.steps.unwrap_or(20),
        guidance_scale: req.guidance_scale.unwrap_or(7.5),
        seed: req.seed,
        scheduler,
    };

    info!("Media IPC: Generating image with prompt '{}'", req.prompt);
    let res = state.diffusion.generate(diff_req).await.map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
pub async fn media_generate_video(req: GenerateVideoDto) -> Result<VideoGenerationResponse, String> {
    let state = MEDIA_STATE.lock().await;
    let vid_req = VideoGenerationRequest {
        prompt: req.prompt.clone(),
        model_id: "ltx-video-2b".to_string(),
        width: req.width.unwrap_or(512),
        height: req.height.unwrap_or(512),
        num_frames: req.num_frames.unwrap_or(24),
        fps: req.fps.unwrap_or(24),
        steps: 20,
        seed: None,
    };

    info!("Media IPC: Generating video with prompt '{}'", req.prompt);
    let res = state.video.generate_video(vid_req).await.map_err(|e| e.to_string())?;
    Ok(res)
}
