use crate::cuda::analyzer::CudaKernel;
use crate::cuda::ptx_parser::PtxAnalysis;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vllm_client::{ChatMessage, ChatRequest, InferenceProvider};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaReconstructionResult {
    pub kernel_name: String,
    pub inferred_algorithm: String,
    pub cuda_source_code: String,
    pub explanation: String,
    pub confidence_score: f32,
}

pub struct NeuralCudaLifter {
    provider: Arc<dyn InferenceProvider>,
    target_model: String,
}

impl NeuralCudaLifter {
    pub fn new(provider: Arc<dyn InferenceProvider>, target_model: Option<String>) -> Self {
        Self {
            provider,
            target_model: target_model.unwrap_or_else(|| "qwen2.5-coder:32b".to_string()),
        }
    }

    pub async fn lift_kernel(
        &self,
        kernel: &CudaKernel,
        analysis: &PtxAnalysis,
    ) -> Result<CudaReconstructionResult> {
        let ptx = kernel.ptx_code.as_deref().unwrap_or("// No PTX found");

        let prompt = format!(
            "You are an expert CUDA C++ performance and reverse engineering specialist.\n\
            Analyze the following PTX (Parallel Thread Execution) extracted from a proprietary deep learning library (e.g., cuDNN).\n\
            \n\
            [Context]\n\
            - Kernel Name: {}\n\
            - Target Architecture: {}\n\
            - Shared Memory: {} bytes\n\
            - Tensor Core Detected: {}\n\
            - Inferred Baseline: {}\n\
            \n\
            [PTX Code]\n\
            {}\n\
            \n\
            [Task]\n\
            1. Identify the high-level mathematical operation (e.g. Implicit GEMM, Winograd Conv, FlashAttention).\n\
            2. Reconstruct the idiomatic CUDA C++ kernel that would compile to this PTX.\n\
            3. Use __half2, __nv_bfloat16, or wmma / mma intrinsics if mixed-precision tensor cores are detected.\n\
            4. Reconstruct the __shared__ memory tile layouts and warp indexing.\n\
            5. Output the reconstructed CUDA C++ kernel in a ```cuda or ```cpp block followed by an architectural breakdown.",
            kernel.name,
            analysis.target_arch,
            analysis.memory_pattern.shared_memory_bytes,
            if !analysis.tensor_core_patterns.is_empty() { "Yes" } else { "No" },
            analysis.inferred_operation,
            ptx
        );

        let req = ChatRequest {
            model: self.target_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are Oxide-Tech Neural CUDA Lifter. Decompile PTX into idiomatic, high-performance CUDA C++.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.1),
            max_tokens: Some(4096),
            json_mode: None,
        };

        let resp = self
            .provider
            .chat_completion(req)
            .await
            .context("Neural CUDA lifting LLM request failed")?;

        let raw_output = resp.content;
        let cuda_code = if let Some(start) = raw_output.find("```cuda") {
            let after = &raw_output[start + 7..];
            if let Some(end) = after.find("```") {
                after[..end].trim().to_string()
            } else {
                after.trim().to_string()
            }
        } else if let Some(start) = raw_output.find("```cpp") {
            let after = &raw_output[start + 6..];
            if let Some(end) = after.find("```") {
                after[..end].trim().to_string()
            } else {
                after.trim().to_string()
            }
        } else {
            raw_output.clone()
        };

        Ok(CudaReconstructionResult {
            kernel_name: kernel.name.clone(),
            inferred_algorithm: analysis.inferred_operation.clone(),
            cuda_source_code: cuda_code,
            explanation: raw_output,
            confidence_score: 0.94,
        })
    }
}
