use crate::cuda::analyzer::CudaKernel;
use crate::cuda::ptx_parser::PtxAnalysis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaKernelNode {
    pub name: String,
    pub arch: String,
    pub ptx_code: String,
    pub sass_code: Option<String>,
    pub registers_used: u32,
    pub shared_memory_bytes: usize,
    pub has_tensor_core: bool,
    pub inferred_operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaMemoryAccessNode {
    pub kernel_id: String,
    pub kind: String, // "global", "shared", "local", "constant"
    pub coalesced: bool,
    pub bank_conflicts: u32,
}

pub struct CudaGraphBridge;

impl CudaGraphBridge {
    pub fn build_kernel_node(kernel: &CudaKernel, analysis: &PtxAnalysis) -> CudaKernelNode {
        CudaKernelNode {
            name: kernel.name.clone(),
            arch: kernel.arch.clone(),
            ptx_code: kernel.ptx_code.clone().unwrap_or_default(),
            sass_code: kernel.sass_code.clone(),
            registers_used: kernel.registers_used.unwrap_or(32),
            shared_memory_bytes: analysis.memory_pattern.shared_memory_bytes,
            has_tensor_core: !analysis.tensor_core_patterns.is_empty(),
            inferred_operation: analysis.inferred_operation.clone(),
        }
    }
}
