use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorCorePattern {
    pub instruction: String,
    pub shape: String,     // e.g. "m16n8k16"
    pub precision: String, // e.g. "f32.tf32.tf32.f32", "f16"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccessPattern {
    pub uses_shared_tiling: bool,
    pub shared_memory_bytes: usize,
    pub uses_coalesced_global: bool,
    pub uses_async_copy: bool, // cp.async (Ampere+)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtxAnalysis {
    pub kernel_name: String,
    pub target_arch: String,
    pub tensor_core_patterns: Vec<TensorCorePattern>,
    pub memory_pattern: MemoryAccessPattern,
    pub inferred_operation: String,
}

pub struct PtxParser;

impl PtxParser {
    pub fn analyze(ptx_code: &str) -> PtxAnalysis {
        let mut target_arch = "sm_50".to_string();
        let mut kernel_name = "unknown_kernel".to_string();
        let mut tensor_core_patterns = Vec::new();
        let mut uses_shared_tiling = false;
        let mut uses_coalesced_global = false;
        let mut uses_async_copy = false;
        let mut shared_memory_bytes = 0;

        let arch_re = Regex::new(r"\.target\s+([a-zA-Z0-9_]+)").unwrap();
        let entry_re = Regex::new(r"\.entry\s+([a-zA-Z0-9_]+)").unwrap();
        let shared_re =
            Regex::new(r"\.shared\s+\.align\s+\d+\s+\.b8\s+[a-zA-Z0-9_]+\[(\d+)\]").unwrap();
        let mma_re =
            Regex::new(r"mma\.sync\.aligned\.([a-zA-Z0-9]+)\.row\.col\.([a-zA-Z0-9\.]+)").unwrap();

        if let Some(caps) = arch_re.captures(ptx_code)
            && let Some(m) = caps.get(1)
        {
            target_arch = m.as_str().to_string();
        }

        if let Some(caps) = entry_re.captures(ptx_code)
            && let Some(m) = caps.get(1)
        {
            kernel_name = m.as_str().to_string();
        }

        if let Some(caps) = shared_re.captures(ptx_code)
            && let Some(m) = caps.get(1)
        {
            shared_memory_bytes = m.as_str().parse::<usize>().unwrap_or(0);
            uses_shared_tiling = true;
        }

        if ptx_code.contains("ld.shared") || ptx_code.contains("st.shared") {
            uses_shared_tiling = true;
        }

        if ptx_code.contains("ld.global") || ptx_code.contains("st.global") {
            uses_coalesced_global = true;
        }

        if ptx_code.contains("cp.async") {
            uses_async_copy = true;
        }

        for caps in mma_re.captures_iter(ptx_code) {
            let shape = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let precision = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            tensor_core_patterns.push(TensorCorePattern {
                instruction: caps
                    .get(0)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                shape,
                precision,
            });
        }

        let inferred_operation = if !tensor_core_patterns.is_empty() {
            if uses_async_copy {
                "Ampere/Hopper Tensor Core Tiled GEMM with Asynchronous Copy (cuDNN / Cutlass)"
                    .to_string()
            } else {
                "Tensor Core Matrix Multiplication (GEMM / Conv2d)".to_string()
            }
        } else if uses_shared_tiling {
            "Shared-Memory Tiled Stencil / Reduction Kernel".to_string()
        } else {
            "Elementwise / Linear Memory Kernel".to_string()
        };

        PtxAnalysis {
            kernel_name,
            target_arch,
            tensor_core_patterns,
            memory_pattern: MemoryAccessPattern {
                uses_shared_tiling,
                shared_memory_bytes,
                uses_coalesced_global,
                uses_async_copy,
            },
            inferred_operation,
        }
    }
}
