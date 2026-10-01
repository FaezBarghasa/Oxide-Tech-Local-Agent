//! media-forge: Pure-Rust native multimodal generative media engine.

pub mod diffusion;
pub mod video;

pub use diffusion::{DiffusionEngine, DiffusionRequest, DiffusionResponse, SchedulerType};
pub use video::{VideoEngine, VideoGenerationRequest, VideoGenerationResponse};

use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct MediaForgeEngine {
    diffusion: Arc<DiffusionEngine>,
    video: Arc<VideoEngine>,
}

impl MediaForgeEngine {
    pub fn new() -> Result<Self> {
        let diffusion = Arc::new(DiffusionEngine::new());
        let video = Arc::new(VideoEngine::new());
        Ok(Self { diffusion, video })
    }

    pub fn diffusion(&self) -> &DiffusionEngine {
        &self.diffusion
    }

    pub fn video(&self) -> &VideoEngine {
        &self.video
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_media_forge_diffusion() {
        let engine = MediaForgeEngine::new().unwrap();
        let req = DiffusionRequest {
            prompt: "cyberpunk PCB trace render".to_string(),
            width: 64,
            height: 64,
            steps: 2,
            ..Default::default()
        };
        let res = engine.diffusion().generate(req).await.unwrap();
        assert_eq!(res.image_width, 64);
        assert_eq!(res.image_height, 64);
        assert_eq!(res.raw_rgb.len(), 64 * 64 * 3);
    }
}
