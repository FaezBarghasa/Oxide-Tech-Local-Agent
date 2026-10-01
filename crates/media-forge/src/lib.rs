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
