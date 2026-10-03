use audio_forge::{
    AudioCaptureRingBuffer, AudioDeviceManager, DeviceInfo, SpeechSynthesisEngine,
    SpeechToTextEngine, SynthesisRequest, TranscriptionResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone, Serialize, Deserialize)]
pub struct AudioDevicesResponse {
    pub default_input: Option<String>,
    pub default_output: Option<String>,
    pub input_devices: Vec<DeviceInfo>,
    pub output_devices: Vec<DeviceInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TtsRequestDto {
    pub text: String,
    pub voice_id: Option<String>,
    pub speed: Option<f32>,
    pub pitch: Option<f32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TtsResponseDto {
    pub audio_base64: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_ms: u64,
}

static AUDIO_STATE: std::sync::LazyLock<Arc<Mutex<AudioState>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(AudioState::new())));

struct AudioState {
    device_mgr: Option<AudioDeviceManager>,
    ring_buf: Arc<AudioCaptureRingBuffer>,
    stt: SpeechToTextEngine,
    tts: SpeechSynthesisEngine,
}

impl AudioState {
    fn new() -> Self {
        let device_mgr = AudioDeviceManager::new().ok();
        let ring_buf = Arc::new(AudioCaptureRingBuffer::new(16000 * 30));
        let stt = SpeechToTextEngine::new();
        let tts = SpeechSynthesisEngine::new();
        Self {
            device_mgr,
            ring_buf,
            stt,
            tts,
        }
    }
}

#[tauri::command]
pub async fn audio_list_devices() -> Result<AudioDevicesResponse, String> {
    let state = AUDIO_STATE.lock().await;
    if let Some(ref mgr) = state.device_mgr {
        let input_devices = mgr.list_input_devices().map_err(|e| e.to_string())?;
        let output_devices = mgr.list_output_devices().map_err(|e| e.to_string())?;
        let default_input = input_devices
            .iter()
            .find(|d| d.is_default)
            .map(|d| d.name.clone());
        let default_output = output_devices
            .iter()
            .find(|d| d.is_default)
            .map(|d| d.name.clone());
        Ok(AudioDevicesResponse {
            default_input,
            default_output,
            input_devices,
            output_devices,
        })
    } else {
        Ok(AudioDevicesResponse {
            default_input: None,
            default_output: None,
            input_devices: vec![],
            output_devices: vec![],
        })
    }
}

#[tauri::command]
pub async fn audio_start_recording() -> Result<bool, String> {
    let state = AUDIO_STATE.lock().await;
    state.ring_buf.clear();
    state.ring_buf.set_recording(true);
    info!("Audio capture started");
    Ok(true)
}

#[tauri::command]
pub async fn audio_stop_and_transcribe() -> Result<TranscriptionResult, String> {
    let state = AUDIO_STATE.lock().await;
    state.ring_buf.set_recording(false);
    let samples = state.ring_buf.get_recent_samples(16000 * 30);
    info!("Transcribing captured audio ({} samples)", samples.len());
    let res = state
        .stt
        .transcribe(&samples)
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
pub async fn audio_synthesize_speech(req: TtsRequestDto) -> Result<TtsResponseDto, String> {
    let state = AUDIO_STATE.lock().await;
    let tts_req = SynthesisRequest {
        text: req.text,
        voice: req.voice_id.unwrap_or_else(|| "kokoro-v1.0".to_string()),
        speed: req.speed.unwrap_or(1.0),
        pitch: req.pitch.unwrap_or(1.0),
    };
    let resp = state
        .tts
        .synthesize(tts_req)
        .await
        .map_err(|e| e.to_string())?;
    let duration_ms = (resp.pcm_data.len() as u64 * 1000) / resp.sample_rate as u64;

    // Encode PCM f32 to raw bytes
    let mut byte_data = Vec::with_capacity(resp.pcm_data.len() * 2);
    for s in &resp.pcm_data {
        let clamped = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        byte_data.extend_from_slice(&clamped.to_le_bytes());
    }
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&byte_data);

    Ok(TtsResponseDto {
        audio_base64: b64,
        sample_rate: resp.sample_rate,
        channels: resp.channels,
        duration_ms,
    })
}
