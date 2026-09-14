use anyhow::Result;
use async_trait::async_trait;
use re_forge::{
    analyzer::{DisassembledFunction, DisassembledInstruction},
    cfg::ControlFlowGraph,
    neural_decompiler::NeuralDecompiler,
};
use std::sync::Arc;
use vllm_client::{
    BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
    ProviderKind, StreamResult,
};

struct MockInferenceProvider;

#[async_trait]
impl InferenceProvider for MockInferenceProvider {
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
            content: "```rust\npub fn calculate_sum(a: i32, b: i32) -> i32 {\n    a + b\n}\n```\nInferred ABI: System V AMD64 calling convention.".to_string(),
            prompt_tokens: 120,
            completion_tokens: 45,
            finish_reason: Some("stop".to_string()),
        })
    }

    async fn stream_chat(&self, _req: ChatRequest) -> Result<StreamResult> {
        unimplemented!()
    }

    async fn health(&self) -> Result<BackendHealth> {
        Ok(BackendHealth {
            healthy: true,
            provider_name: "MockProvider".to_string(),
            active_model: "mock-model".to_string(),
            memory_used_mb: Some(0),
        })
    }
}

#[tokio::test]
async fn test_control_flow_graph_and_decompilation() {
    let instructions = vec![
        DisassembledInstruction {
            address: 0x1000,
            mnemonic: "mov eax, edi".to_string(),
            length: 2,
            is_branch: false,
            is_call: false,
            is_return: false,
        },
        DisassembledInstruction {
            address: 0x1002,
            mnemonic: "add eax, esi".to_string(),
            length: 2,
            is_branch: false,
            is_call: false,
            is_return: false,
        },
        DisassembledInstruction {
            address: 0x1004,
            mnemonic: "ret".to_string(),
            length: 1,
            is_branch: false,
            is_call: false,
            is_return: true,
        },
    ];

    let func = DisassembledFunction {
        name: "add_numbers".to_string(),
        start_address: 0x1000,
        end_address: 0x1005,
        instructions,
    };

    let cfg = ControlFlowGraph::from_function(&func);
    assert_eq!(cfg.block_count(), 1);

    let provider = Arc::new(MockInferenceProvider);
    let decompiler = NeuralDecompiler::new(provider, Some("qwen2.5-coder:7b".to_string()));

    let result = decompiler.decompile_function(&func, &cfg).await.unwrap();
    assert_eq!(result.function_name, "add_numbers");
    assert!(result.rust_source_code.contains("pub fn calculate_sum"));
    assert!(result.explanation.contains("System V AMD64"));
}
