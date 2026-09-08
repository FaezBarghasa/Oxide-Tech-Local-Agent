use serde::{Deserialize, Serialize};

/// High-throughput, `#![no_std]` binary compatible telemetry packet for microcontrollers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardwareTelemetryPacket {
    pub device_id: u32,
    pub timestamp_ms: u64,
    pub cpu_frequency_mhz: u16,
    pub vdda_millivolts: u16,
    pub core_temperature_c: f32,
    pub rtt_log_snippet: String,
}

impl HardwareTelemetryPacket {
    /// Zero-heap binary serialization via postcard for direct UART / RTT streaming.
    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActuationCommand {
    ResetTarget,
    HaltCore,
    ResumeCore,
    SetGpio { pin: u8, state: bool },
    TriggerDacVoltage { channel: u8, millivolts: u16 },
}
