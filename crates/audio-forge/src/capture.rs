//! Microphone capture and circular PCM ring buffering via CPAL.

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct AudioDeviceManager {
    host: cpal::Host,
}

impl AudioDeviceManager {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        Ok(Self { host })
    }

    pub fn list_input_devices(&self) -> Result<Vec<DeviceInfo>> {
        let default_name = self.host.default_input_device().and_then(|d| d.name().ok());
        let mut devices = Vec::new();

        if let Ok(input_devices) = self.host.input_devices() {
            for dev in input_devices {
                if let Ok(name) = dev.name() {
                    let is_default = default_name.as_ref().map(|d| d == &name).unwrap_or(false);
                    let (sample_rate, channels) = if let Ok(conf) = dev.default_input_config() {
                        (conf.sample_rate().0, conf.channels())
                    } else {
                        (16000, 1)
                    };

                    devices.push(DeviceInfo {
                        name,
                        is_default,
                        sample_rate,
                        channels,
                    });
                }
            }
        }
        Ok(devices)
    }

    pub fn list_output_devices(&self) -> Result<Vec<DeviceInfo>> {
        let default_name = self.host.default_output_device().and_then(|d| d.name().ok());
        let mut devices = Vec::new();

        if let Ok(output_devices) = self.host.output_devices() {
            for dev in output_devices {
                if let Ok(name) = dev.name() {
                    let is_default = default_name.as_ref().map(|d| d == &name).unwrap_or(false);
                    let (sample_rate, channels) = if let Ok(conf) = dev.default_output_config() {
                        (conf.sample_rate().0, conf.channels())
                    } else {
                        (44100, 2)
                    };

                    devices.push(DeviceInfo {
                        name,
                        is_default,
                        sample_rate,
                        channels,
                    });
                }
            }
        }
        Ok(devices)
    }
}

pub struct AudioCaptureRingBuffer {
    capacity: usize,
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<AtomicBool>,
}

impl AudioCaptureRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            is_recording: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn push_samples(&self, samples: &[f32]) {
        let mut buf = self.buffer.lock().unwrap();
        for &s in samples {
            if buf.len() >= self.capacity {
                buf.remove(0);
            }
            buf.push(s);
        }
    }

    pub fn get_recent_samples(&self, count: usize) -> Vec<f32> {
        let buf = self.buffer.lock().unwrap();
        let len = buf.len();
        if len <= count {
            buf.clone()
        } else {
            buf[len - count..].to_vec()
        }
    }

    pub fn clear(&self) {
        let mut buf = self.buffer.lock().unwrap();
        buf.clear();
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }

    pub fn set_recording(&self, active: bool) {
        self.is_recording.store(active, Ordering::Relaxed);
    }
}
