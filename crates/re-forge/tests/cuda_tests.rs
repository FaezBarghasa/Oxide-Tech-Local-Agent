use anyhow::Result;
use async_trait::async_trait;
use re_forge::{CudaAnalyzer, CudaGraphBridge, CudaKernel, NeuralCudaLifter, PtxParser};
use std::sync::Arc;
use vllm_client::{
    BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
    ProviderKind, StreamResult,
};

struct MockCudaInferenceProvider;

#[async_trait]
impl InferenceProvider for MockCudaInferenceProvider {
    fn capabilities(&self) -> InferenceCapabilities {
        InferenceCapabilities {
            provider: ProviderKind::Ollama,
            supports_streaming: false,
            supports_tools: false,
            supports_lora_hotswap: false,
            supports_json_mode: false,
            supports_vision: false,
            max_context_tokens: 8192,
        }
    }

    async fn chat_completion(&self, _req: ChatRequest) -> Result<ChatResponse> {
        Ok(ChatResponse {
            content: "```cuda\n__global__ void cudnn_tiled_gemm(const half* __restrict__ A, const half* __restrict__ B, float* __restrict__ C) {\n    __shared__ half tile_A[16][16];\n    __shared__ half tile_B[16][16];\n    // Reconstructed WMMA Tensor Core GEMM\n}\n```\nExplanation: Implicit GEMM kernel utilizing Ampere Tensor Cores (mma.sync).".to_string(),
            prompt_tokens: 250,
            completion_tokens: 70,
            finish_reason: Some("stop".to_string()),
        })
    }

    async fn stream_chat(&self, _req: ChatRequest) -> Result<StreamResult> {
        unimplemented!()
    }

    async fn health(&self) -> Result<BackendHealth> {
        Ok(BackendHealth {
            healthy: true,
            provider_name: "MockCudaProvider".to_string(),
            active_model: "qwen2.5-coder:32b".to_string(),
            memory_used_mb: Some(0),
        })
    }
}

#[tokio::test]
async fn test_cuda_ptx_tensor_core_analysis_and_lifting() {
    let sample_ptx = r#"
.version 7.0
.target sm_80
.address_size 64

.entry cudnn_tiled_gemm_kernel (
    .param .u64 param_A,
    .param .u64 param_B,
    .param .u64 param_C
) {
    .shared .align 16 .b8 smem_tile[16384];
    ld.global.nc.f32 %f1, [%rd1];
    cp.async.ca.shared.global [%r1], [%rd2], 16;
    mma.sync.aligned.m16n8k16.row.col.f32.tf32.tf32.f32 {%f1, %f2, %f3, %f4}, {%r1, %r2, %r3, %r4}, {%r5, %r6}, {%f1, %f2, %f3, %f4};
    st.global.f32 [%rd3], %f1;
    ret;
}
"#;

    let analysis = PtxParser::analyze(sample_ptx);
    assert_eq!(analysis.kernel_name, "cudnn_tiled_gemm_kernel");
    assert_eq!(analysis.target_arch, "sm_80");
    assert_eq!(analysis.memory_pattern.shared_memory_bytes, 16384);
    assert!(analysis.memory_pattern.uses_shared_tiling);
    assert!(analysis.memory_pattern.uses_async_copy);
    assert_eq!(analysis.tensor_core_patterns.len(), 1);
    assert_eq!(analysis.tensor_core_patterns[0].shape, "m16n8k16");

    let analyzer = CudaAnalyzer::new(false);
    let extracted = analyzer.extract_kernels(sample_ptx.as_bytes()).unwrap();
    assert!(!extracted.is_empty());

    let kernel = CudaKernel {
        name: analysis.kernel_name.clone(),
        arch: analysis.target_arch.clone(),
        ptx_code: Some(sample_ptx.to_string()),
        sass_code: None,
        registers_used: Some(64),
        shared_memory_bytes: Some(16384),
    };

    let node = CudaGraphBridge::build_kernel_node(&kernel, &analysis);
    assert_eq!(node.name, "cudnn_tiled_gemm_kernel");
    assert!(node.has_tensor_core);

    let provider = Arc::new(MockCudaInferenceProvider);
    let lifter = NeuralCudaLifter::new(provider, Some("qwen2.5-coder:32b".to_string()));

    let result = lifter.lift_kernel(&kernel, &analysis).await.unwrap();
    assert_eq!(result.kernel_name, "cudnn_tiled_gemm_kernel");
    assert!(
        result
            .cuda_source_code
            .contains("__global__ void cudnn_tiled_gemm")
    );
    assert!(result.explanation.contains("Implicit GEMM"));
}
