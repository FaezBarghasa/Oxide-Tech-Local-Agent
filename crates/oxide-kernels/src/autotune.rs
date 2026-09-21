use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// Targeted Nvidia Compute Architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NvidiaArch {
    AmpereSm80,
    AmpereSm86,
    AdaSm89,
    HopperSm90,
    BlackwellSm100,
    BlackwellSm120,
    FallbackGeneric,
}

impl NvidiaArch {
    /// Detect architecture from device compute capability (major, minor)
    pub fn from_compute_capability(major: u32, minor: u32) -> Self {
        match (major, minor) {
            (8, 0) => NvidiaArch::AmpereSm80,
            (8, 6) | (8, 7) => NvidiaArch::AmpereSm86,
            (8, 9) => NvidiaArch::AdaSm89,
            (9, 0) => NvidiaArch::HopperSm90,
            (10, 0) => NvidiaArch::BlackwellSm100,
            (12, 0) => NvidiaArch::BlackwellSm120,
            _ => NvidiaArch::FallbackGeneric,
        }
    }

    /// Maximum shared memory per SM in bytes
    pub fn max_shared_memory_bytes(&self) -> usize {
        match self {
            NvidiaArch::AmpereSm80 => 164 * 1024,
            NvidiaArch::AmpereSm86 => 100 * 1024,
            NvidiaArch::AdaSm89 => 100 * 1024,
            NvidiaArch::HopperSm90 => 228 * 1024,
            NvidiaArch::BlackwellSm100 | NvidiaArch::BlackwellSm120 => 256 * 1024,
            NvidiaArch::FallbackGeneric => 64 * 1024,
        }
    }

    /// Whether Tensor Memory Accelerator (TMA) is natively supported
    pub fn supports_tma(&self) -> bool {
        matches!(
            self,
            NvidiaArch::HopperSm90 | NvidiaArch::BlackwellSm100 | NvidiaArch::BlackwellSm120
        )
    }

    /// Whether native FP8 / FP4 tensor cores are available
    pub fn supports_fp8_mma(&self) -> bool {
        matches!(
            self,
            NvidiaArch::AdaSm89
                | NvidiaArch::HopperSm90
                | NvidiaArch::BlackwellSm100
                | NvidiaArch::BlackwellSm120
        )
    }
}

/// Fused Kernel Tile & Pipeline Configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelConfig {
    pub block_m: usize,
    pub block_n: usize,
    pub block_k: usize,
    pub num_warps: usize,
    pub num_stages: usize,
    pub use_tma: bool,
    pub use_fp8: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            block_m: 64,
            block_n: 64,
            block_k: 32,
            num_warps: 4,
            num_stages: 2,
            use_tma: false,
            use_fp8: false,
        }
    }
}

/// Autotuner Cache Key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelKey {
    pub arch: NvidiaArch,
    pub m: usize,
    pub n: usize,
    pub k: usize,
    pub is_fp8: bool,
}

/// Fused GPU Kernel Autotuner
pub struct GpuAutotuner {
    arch: NvidiaArch,
    cache: RwLock<HashMap<KernelKey, KernelConfig>>,
}

impl GpuAutotuner {
    pub fn new(arch: NvidiaArch) -> Self {
        Self {
            arch,
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn current_arch(&self) -> NvidiaArch {
        self.arch
    }

    /// Select optimal kernel tile size and pipeline configuration
    pub fn get_optimal_config(&self, m: usize, n: usize, k: usize, is_fp8: bool) -> KernelConfig {
        let key = KernelKey {
            arch: self.arch,
            m,
            n,
            k,
            is_fp8,
        };

        if let Ok(guard) = self.cache.read()
            && let Some(config) = guard.get(&key)
        {
            return *config;
        }

        let config = self.derive_heuristic_config(m, n, k, is_fp8);

        if let Ok(mut guard) = self.cache.write() {
            guard.insert(key, config);
        }

        config
    }

    fn derive_heuristic_config(&self, m: usize, n: usize, k: usize, is_fp8: bool) -> KernelConfig {
        let use_tma = self.arch.supports_tma();
        let use_fp8 = is_fp8 && self.arch.supports_fp8_mma();

        match self.arch {
            NvidiaArch::BlackwellSm100 | NvidiaArch::BlackwellSm120 => {
                let block_m = if m >= 256 {
                    256
                } else if m >= 128 {
                    128
                } else {
                    64
                };
                let block_n = if n >= 256 {
                    256
                } else if n >= 128 {
                    128
                } else {
                    64
                };
                let block_k = if k >= 128 { 128 } else { 64 };
                KernelConfig {
                    block_m,
                    block_n,
                    block_k,
                    num_warps: if block_m * block_n >= 256 * 128 {
                        16
                    } else {
                        8
                    },
                    num_stages: 5,
                    use_tma,
                    use_fp8,
                }
            }
            NvidiaArch::HopperSm90 => {
                let block_m = if m >= 128 { 128 } else { 64 };
                let block_n = if n >= 128 { 128 } else { 64 };
                let block_k = if k >= 64 { 64 } else { 32 };
                KernelConfig {
                    block_m,
                    block_n,
                    block_k,
                    num_warps: 8,
                    num_stages: 4,
                    use_tma,
                    use_fp8,
                }
            }
            NvidiaArch::AdaSm89 => {
                let block_m = if m >= 128 { 128 } else { 64 };
                let block_n = if n >= 64 { 64 } else { 32 };
                let block_k = 32;
                KernelConfig {
                    block_m,
                    block_n,
                    block_k,
                    num_warps: 4,
                    num_stages: 3,
                    use_tma: false,
                    use_fp8,
                }
            }
            NvidiaArch::AmpereSm80 | NvidiaArch::AmpereSm86 => {
                let block_m = if m >= 128 { 128 } else { 64 };
                let block_n = if n >= 64 { 64 } else { 32 };
                let block_k = 32;
                KernelConfig {
                    block_m,
                    block_n,
                    block_k,
                    num_warps: 4,
                    num_stages: 3,
                    use_tma: false,
                    use_fp8: false,
                }
            }
            NvidiaArch::FallbackGeneric => KernelConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_detection_and_capabilities() {
        let hopper = NvidiaArch::from_compute_capability(9, 0);
        assert_eq!(hopper, NvidiaArch::HopperSm90);
        assert!(hopper.supports_tma());
        assert!(hopper.supports_fp8_mma());
        assert_eq!(hopper.max_shared_memory_bytes(), 228 * 1024);

        let ampere = NvidiaArch::from_compute_capability(8, 6);
        assert_eq!(ampere, NvidiaArch::AmpereSm86);
        assert!(!ampere.supports_tma());
        assert!(!ampere.supports_fp8_mma());
    }

    #[test]
    fn test_autotune_config_heuristics() {
        let tuner = GpuAutotuner::new(NvidiaArch::HopperSm90);
        let config = tuner.get_optimal_config(4096, 4096, 4096, true);
        assert_eq!(config.block_m, 128);
        assert_eq!(config.block_n, 128);
        assert_eq!(config.num_stages, 4);
        assert!(config.use_tma);
        assert!(config.use_fp8);

        // Cache hit test
        let config_cached = tuner.get_optimal_config(4096, 4096, 4096, true);
        assert_eq!(config, config_cached);
    }
}
