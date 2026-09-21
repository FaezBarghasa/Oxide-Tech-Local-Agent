use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Circuit breaker state for admission control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    /// Safe headroom; all work admitted
    Open,
    /// Approaching thresholds; warn operator but admit work
    Warning,
    /// Critical exhaustion; freeze execution, pause uploads, reject new sessions
    Frozen,
}

/// Rejection reason when work admission fails
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateRejection {
    DiskSpaceCritical {
        available_mb: u64,
        min_required_mb: u64,
    },
    VramExhausted {
        available_mb: u64,
        min_required_mb: u64,
    },
}

/// Dynamic Disk and VRAM circuit breaker
pub struct ResourceGater {
    mount_path: PathBuf,
    min_disk_free_mb: u64,
    resume_disk_free_mb: u64,
    min_vram_free_mb: u64,
    status: Arc<RwLock<GateStatus>>,
}

impl Default for ResourceGater {
    fn default() -> Self {
        Self::new("/", 2048, 2560, 1024) // < 2GB freeze, > 2.5GB resume, < 1GB VRAM freeze
    }
}

impl ResourceGater {
    pub fn new(
        mount_path: impl AsRef<Path>,
        min_disk_free_mb: u64,
        resume_disk_free_mb: u64,
        min_vram_free_mb: u64,
    ) -> Self {
        Self {
            mount_path: mount_path.as_ref().to_path_buf(),
            min_disk_free_mb,
            resume_disk_free_mb,
            min_vram_free_mb,
            status: Arc::new(RwLock::new(GateStatus::Open)),
        }
    }

    /// Read available disk space on the target filesystem in Megabytes using sysinfo
    pub fn get_available_disk_mb(&self) -> u64 {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        for disk in &disks {
            if self.mount_path.starts_with(disk.mount_point()) {
                return disk.available_space() / (1024 * 1024);
            }
        }
        // Fallback to first available disk or default
        disks
            .first()
            .map(|d| d.available_space() / (1024 * 1024))
            .unwrap_or(10_000)
    }

    /// Current gate status
    pub async fn current_status(&self) -> GateStatus {
        *self.status.read().await
    }

    /// Poll system metrics, updating hysteresis circuit breaker
    pub async fn poll_and_update(&self, free_vram_mb: Option<u64>) -> GateStatus {
        let free_disk_mb = self.get_available_disk_mb();
        let mut guard = self.status.write().await;
        let prev = *guard;

        let is_disk_critical = free_disk_mb < self.min_disk_free_mb;
        let is_vram_critical = free_vram_mb.is_some_and(|v| v < self.min_vram_free_mb);

        if is_disk_critical || is_vram_critical {
            *guard = GateStatus::Frozen;
            if prev != GateStatus::Frozen {
                tracing::warn!(
                    target: "resource_gater",
                    "Resource gate tripped to FROZEN (Free disk: {} MB, Free VRAM: {:?} MB)",
                    free_disk_mb,
                    free_vram_mb
                );
            }
        } else if free_disk_mb > self.resume_disk_free_mb {
            *guard = GateStatus::Open;
            if prev == GateStatus::Frozen {
                tracing::info!(
                    target: "resource_gater",
                    "Resource gate recovered to OPEN (Free disk: {} MB)",
                    free_disk_mb
                );
            }
        } else if free_disk_mb < self.resume_disk_free_mb {
            *guard = GateStatus::Warning;
        }

        *guard
    }

    /// Check admission before accepting new agent turns or heavy allocations
    pub async fn admit_work(&self, free_vram_mb: Option<u64>) -> Result<(), GateRejection> {
        let free_disk_mb = self.get_available_disk_mb();
        if free_disk_mb < self.min_disk_free_mb {
            return Err(GateRejection::DiskSpaceCritical {
                available_mb: free_disk_mb,
                min_required_mb: self.min_disk_free_mb,
            });
        }

        if let Some(vram_mb) = free_vram_mb
            && vram_mb < self.min_vram_free_mb
        {
            return Err(GateRejection::VramExhausted {
                available_mb: vram_mb,
                min_required_mb: self.min_vram_free_mb,
            });
        }

        let status = *self.status.read().await;
        if status == GateStatus::Frozen {
            return Err(GateRejection::DiskSpaceCritical {
                available_mb: free_disk_mb,
                min_required_mb: self.resume_disk_free_mb,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_gater_hysteresis() {
        let gater = ResourceGater::new("/", 2000, 2500, 1000);

        // Under min disk -> frozen
        let status = gater.poll_and_update(Some(500)).await;
        assert_eq!(status, GateStatus::Frozen);

        // VRAM failure check
        assert!(gater.admit_work(Some(500)).await.is_err());
    }
}
