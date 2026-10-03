//! System Controller
//!
//! Consolidates hardware telemetry, SWD probe-rs flasher, doctor diagnostics,
//! udev installation, embedded gateway status, cloudflare tunnels, configuration,
//! and OTA updater.

pub use crate::config_ipc::*;
pub use crate::doctor::*;
pub use crate::hardware_ipc::*;
pub use crate::tunnel_ipc::*;
pub use crate::updater_ipc::*;

/// Probe local or remote gateway server liveness.
#[tauri::command]
pub async fn gateway_status(base_url: Option<String>) -> Result<u16, String> {
    let base = base_url.unwrap_or_else(|| crate::DEFAULT_GATEWAY_URL.to_string());
    crate::gateway_rt::probe_gateway(&base, 3).await
}
