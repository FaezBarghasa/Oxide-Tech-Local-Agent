use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeDeviceDto {
    pub identifier: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
    pub product_name: Option<String>,
    pub manufacturer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeDevicesResult {
    pub devices: Vec<ProbeDeviceDto>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashRequest {
    pub device_identifier: String,
    pub firmware_path: String,
    pub chip_name: Option<String>,
    pub verify: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashResult {
    pub success: bool,
    pub message: String,
    pub bytes_written: Option<usize>,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChipInfoDto {
    pub name: String,
    pub part: String,
    pub cores: Vec<CoreInfoDto>,
    pub memory_regions: Vec<MemoryRegionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreInfoDto {
    pub name: String,
    pub core_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryRegionDto {
    pub name: String,
    pub range_start: u64,
    pub range_end: u64,
    pub is_flash: bool,
    pub is_ram: bool,
}

pub fn list_probe_devices() -> Result<ProbeDevicesResult> {
    #[cfg(feature = "probe-rs")]
    {
        use probe_rs::{DebugProbeInfo, Probe};
        let probes = DebugProbeInfo::get_all()
            .context("Failed to list debug probes")?;

        let devices = probes
            .into_iter()
            .map(|p| ProbeDeviceDto {
                identifier: p.identifier.clone(),
                vendor_id: p.vendor_id,
                product_id: p.product_id,
                serial_number: p.serial_number,
                product_name: p.product_name,
                manufacturer: p.manufacturer,
            })
            .collect();

        Ok(ProbeDevicesResult {
            devices,
            error: None,
        })
    }
    #[cfg(not(feature = "probe-rs"))]
    {
        Ok(ProbeDevicesResult {
            devices: Vec::new(),
            error: Some("probe-rs feature not enabled in this build".to_string()),
        })
    }
}

pub fn get_chip_info(_device_identifier: String) -> Result<ChipInfoDto> {
    #[cfg(feature = "probe-rs")]
    {
        use probe_rs::{DebugProbeInfo, Probe, Permissions};
        let probe = DebugProbeInfo::get_all()
            .context("Failed to list debug probes")?
            .into_iter()
            .find(|p| p.identifier == device_identifier)
            .ok_or_else(|| anyhow::anyhow!("Probe not found: {}", device_identifier))?
            .open()
            .context("Failed to open probe")?;

        let target = probe
            .attach(probe_rs::config::ChipInfo::default())
            .context("Failed to attach to target")?;

        let chip_info = target.chip_info();

        let cores = chip_info
            .cores
            .iter()
            .map(|c| CoreInfoDto {
                name: c.name.clone(),
                core_type: format!("{:?}", c.core_type),
            })
            .collect();

        let memory_regions = chip_info
            .memory_map
            .iter()
            .map(|m| MemoryRegionDto {
                name: m.name.clone(),
                range_start: m.range.start,
                range_end: m.range.end,
                is_flash: m.is_flash(),
                is_ram: m.is_ram(),
            })
            .collect();

        Ok(ChipInfoDto {
            name: chip_info.name.clone(),
            part: chip_info.part.clone(),
            cores,
            memory_regions,
        })
    }
    #[cfg(not(feature = "probe-rs"))]
    {
        Err(anyhow::anyhow!("probe-rs feature not enabled"))
    }
}

pub fn flash_firmware(_request: FlashRequest) -> Result<FlashResult> {
    #[cfg(feature = "probe-rs")]
    {
        use probe_rs::{DebugProbeInfo, Probe, Permissions};
        use std::time::Instant;

        let t0 = Instant::now();

        let probe = DebugProbeInfo::get_all()
            .context("Failed to list debug probes")?
            .into_iter()
            .find(|p| p.identifier == request.device_identifier)
            .ok_or_else(|| anyhow::anyhow!("Probe not found: {}", request.device_identifier))?
            .open()
            .context("Failed to open probe")?;

        let mut session = probe
            .attach(probe_rs::config::ChipInfo::default())
            .context("Failed to attach to target")?;

        let firmware_data = std::fs::read(&request.firmware_path)
            .context("Failed to read firmware file")?;

        let chip = if let Some(chip_name) = request.chip_name {
            probe_rs::config::ChipInfo::from_chip_name(&chip_name)
        } else {
            probe_rs::config::ChipInfo::default()
        };

        let target = session
            .target()
            .context("Failed to get target")?;

        target
            .flash_erase_all()
            .context("Failed to erase flash")?;

        let bytes_written = target
            .flash_write(&firmware_data)
            .context("Failed to flash firmware")?;

        if request.verify.unwrap_or(true) {
            target
                .flash_verify(&firmware_data)
                .context("Flash verification failed")?;
        }

        Ok(FlashResult {
            success: true,
            message: "Firmware flashed successfully".to_string(),
            bytes_written: Some(bytes_written),
            duration_ms: Some(t0.elapsed().as_millis() as u64),
        })
    }
    #[cfg(not(feature = "probe-rs"))]
    {
        Err(anyhow::anyhow!("probe-rs feature not enabled"))
    }
}