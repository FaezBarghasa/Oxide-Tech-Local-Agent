//! audio-forge: Pure-Rust audio capture, STT, and TTS engine.

pub mod capture;
pub mod stt;
pub mod tts;

pub use capture::{AudioCaptureRingBuffer, AudioDeviceManager, DeviceInfo};
pub use stt::{SpeechToTextEngine, TranscriptionResult};
pub use tts::{SpeechSynthesisEngine, SynthesisRequest};

use anyhow::Result;
use std::sync::Arc;

/// Master audio orchestration controller.
#[derive(Clone)]
pub struct AudioEngine {
    device_manager: Arc<AudioDeviceManager>,
    capture_buffer: Arc<AudioCaptureRingBuffer>,
    stt: Arc<SpeechToTextEngine>,
    tts: Arc<SpeechSynthesisEngine>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let device_manager = Arc::new(AudioDeviceManager::new()?);
        let capture_buffer = Arc::new(AudioCaptureRingBuffer::new(16000 * 30)); // 30s @ 16kHz
        let stt = Arc::new(SpeechToTextEngine::new());
        let tts = Arc::new(SpeechSynthesisEngine::new());

        Ok(Self {
            device_manager,
            capture_buffer,
            stt,
            tts,
        })
    }

    pub fn device_manager(&self) -> &AudioDeviceManager {
        &self.device_manager
    }

    pub fn capture_buffer(&self) -> &AudioCaptureRingBuffer {
        &self.capture_buffer
    }

    pub fn stt(&self) -> &SpeechToTextEngine {
        &self.stt
    }

    pub fn tts(&self) -> &SpeechSynthesisEngine {
        &self.tts
    }
}
