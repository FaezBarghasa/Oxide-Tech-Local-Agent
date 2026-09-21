use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Storage tier for tensor or KV-cache chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTier {
    /// High-bandwidth GPU VRAM (GDDR6X / HBM3e)
    GpuVram,
    /// Fast Host RAM (DDR5 / LPDDR5X, pinned via mmap or aligned host pages)
    HostDdr5,
    /// NVMe / Disk swap tier
    NvmeDisk,
}

/// Metadata and storage descriptor for offloaded or tier-managed tensors.
#[derive(Debug, Clone)]
pub struct TieredBuffer {
    pub name: String,
    pub tier: MemoryTier,
    pub size_bytes: usize,
    /// Host DDR5 pinned memory buffer (page-locked / aligned for fast PCIe transfer)
    pub host_buffer: Option<Vec<u8>>,
    /// Device VRAM pointer (represented as device address or virtual token)
    pub device_ptr: Option<u64>,
}

/// Dynamic DDR5 RAM Tier Manager that automatically offloads layers, optimizer states,
/// and KV-cache blocks when GPU VRAM hits threshold.
pub struct Ddr5TierManager {
    /// Maximum VRAM threshold in MB before triggering DDR5 offload (e.g., 90% of GPU capacity)
    pub vram_headroom_threshold_mb: u64,
    /// Pinned DDR5 host memory capacity budget in MB
    pub max_ddr5_budget_mb: u64,
    /// Active tracked buffers across memory tiers
    pub buffers: Arc<RwLock<HashMap<String, TieredBuffer>>>,
}

impl Default for Ddr5TierManager {
    fn default() -> Self {
        Self::new(1024, 64 * 1024) // 1GB VRAM headroom trigger, 64GB DDR5 host capacity
    }
}

impl Ddr5TierManager {
    pub fn new(vram_headroom_threshold_mb: u64, max_ddr5_budget_mb: u64) -> Self {
        Self {
            vram_headroom_threshold_mb,
            max_ddr5_budget_mb,
            buffers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Calculate total current DDR5 host memory allocation in Megabytes.
    pub async fn current_ddr5_usage_mb(&self) -> u64 {
        let guard = self.buffers.read().await;
        let bytes: usize = guard
            .values()
            .filter(|b| b.tier == MemoryTier::HostDdr5)
            .map(|b| b.size_bytes)
            .sum();
        (bytes / (1024 * 1024)) as u64
    }

    /// Register a buffer in GPU VRAM
    pub async fn register_gpu_buffer(&self, name: &str, size_bytes: usize, device_ptr: u64) {
        let mut guard = self.buffers.write().await;
        guard.insert(
            name.to_string(),
            TieredBuffer {
                name: name.to_string(),
                tier: MemoryTier::GpuVram,
                size_bytes,
                host_buffer: None,
                device_ptr: Some(device_ptr),
            },
        );
    }

    /// Check if GPU VRAM pressure requires offload to DDR5 RAM.
    pub fn should_offload_to_ddr5(&self, free_vram_mb: u64) -> bool {
        free_vram_mb <= self.vram_headroom_threshold_mb
    }

    /// Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.
    /// In actual execution, this copies device memory over PCIe Gen4/Gen5 into aligned host pages.
    pub async fn offload_to_ddr5(
        &self,
        name: &str,
        data_from_device: Option<Vec<u8>>,
    ) -> Result<MemoryTier, String> {
        let mut guard = self.buffers.write().await;
        let needed_mb = {
            let buf = guard
                .get(name)
                .ok_or_else(|| format!("Buffer '{}' not found in tier manager", name))?;
            if buf.tier == MemoryTier::HostDdr5 {
                return Ok(MemoryTier::HostDdr5);
            }
            (buf.size_bytes / (1024 * 1024)) as u64
        };

        let current_usage_mb: u64 = guard
            .values()
            .filter(|b| b.tier == MemoryTier::HostDdr5)
            .map(|b| b.size_bytes as u64 / (1024 * 1024))
            .sum();

        if current_usage_mb + needed_mb > self.max_ddr5_budget_mb {
            return Err(format!(
                "DDR5 host allocation limit exceeded: {} MB + {} MB > {} MB budget",
                current_usage_mb, needed_mb, self.max_ddr5_budget_mb
            ));
        }

        let buf = guard
            .get_mut(name)
            .ok_or_else(|| format!("Buffer '{}' not found in tier manager", name))?;

        // Move to Host DDR5
        buf.tier = MemoryTier::HostDdr5;
        buf.device_ptr = None;
        buf.host_buffer = data_from_device.or_else(|| Some(vec![0u8; buf.size_bytes]));

        tracing::info!(
            target: "ddr5_tier_manager",
            "Offloaded buffer '{}' ({} MB) to DDR5 host memory over PCIe bus",
            name,
            needed_mb
        );

        Ok(MemoryTier::HostDdr5)
    }

    /// Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.
    pub async fn prefetch_to_vram(
        &self,
        name: &str,
        new_device_ptr: u64,
    ) -> Result<Vec<u8>, String> {
        let mut guard = self.buffers.write().await;
        let buf = guard
            .get_mut(name)
            .ok_or_else(|| format!("Buffer '{}' not found in tier manager", name))?;

        if buf.tier == MemoryTier::GpuVram {
            return Err(format!("Buffer '{}' is already in GPU VRAM", name));
        }

        let host_data = buf
            .host_buffer
            .take()
            .unwrap_or_else(|| vec![0u8; buf.size_bytes]);

        buf.tier = MemoryTier::GpuVram;
        buf.device_ptr = Some(new_device_ptr);

        tracing::debug!(
            target: "ddr5_tier_manager",
            "Prefetched buffer '{}' back into GPU VRAM at 0x{:x}",
            name,
            new_device_ptr
        );

        Ok(host_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ddr5_offloading_and_prefetch_cycle() {
        let manager = Ddr5TierManager::new(1000, 8192);

        // 1. Should offload if free VRAM is under threshold
        assert!(manager.should_offload_to_ddr5(800));
        assert!(!manager.should_offload_to_ddr5(2500));

        // 2. Register 64MB buffer on GPU
        let size = 64 * 1024 * 1024;
        manager
            .register_gpu_buffer("layer.12.weights", size, 0x1000)
            .await;

        // 3. Offload to DDR5
        let dummy_data = vec![42u8; size];
        let tier = manager
            .offload_to_ddr5("layer.12.weights", Some(dummy_data))
            .await
            .unwrap();
        assert_eq!(tier, MemoryTier::HostDdr5);
        assert_eq!(manager.current_ddr5_usage_mb().await, 64);

        // 4. Prefetch back to GPU
        let prefetched = manager
            .prefetch_to_vram("layer.12.weights", 0x2000)
            .await
            .unwrap();
        assert_eq!(prefetched.len(), size);
        assert_eq!(prefetched[0], 42);
        assert_eq!(manager.current_ddr5_usage_mb().await, 0);
    }
}
