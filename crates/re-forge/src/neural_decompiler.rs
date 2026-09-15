use crate::analyzer::DisassembledFunction;
use crate::cfg::ControlFlowGraph;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vllm_client::{ChatMessage, ChatRequest, InferenceProvider};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompilationResult {
    pub function_name: String,
    pub original_assembly: String,
    pub rust_source_code: String,
    pub explanation: String,
    pub confidence_score: f32,
}

pub struct NeuralDecompiler {
    provider: Arc<dyn InferenceProvider>,
    target_model: String,
}

impl NeuralDecompiler {
    pub fn new(provider: Arc<dyn InferenceProvider>, target_model: Option<String>) -> Self {
        Self {
            provider,
            target_model: target_model.unwrap_or_else(|| "qwen2.5-coder:32b".to_string()),
        }
    }

    pub async fn decompile_function(
        &self,
        func: &DisassembledFunction,
        cfg: &ControlFlowGraph,
    ) -> Result<DecompilationResult> {
        let mut asm_listing = String::new();
        for inst in &func.instructions {
            asm_listing.push_str(&format!("  0x{:08x}: {}\n", inst.address, inst.mnemonic));
        }

        let prompt = format!(
            "You are an expert systems and reverse engineering agent.\n\
            Translate the following x86_64 disassembly into idiomatic, safe, and compile-ready Rust.\n\
            Function Name: {}\n\
            Basic Block Count: {}\n\
            \n\
            --- Disassembly ---\n\
            {}\n\
            -------------------\n\
            \n\
            Guidelines:\n\
            1. Output clean Rust syntax with typed function signatures and structs where appropriate.\n\
            2. Infer high-level semantics rather than mechanical pointer arithmetic.\n\
            3. Return only the Rust code block followed by a brief technical analysis of the inferred ABI.",
            func.name,
            cfg.block_count(),
            asm_listing
        );

        let req = ChatRequest {
            model: self.target_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are Oxide-Tech Neural Decompiler. Translate machine assembly to idiomatic Rust.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.1),
            max_tokens: Some(4096),
            json_mode: None,
            history: vec![],
            tools: None,
            tool_choice: None,
            images: None,
            grammar: None,
            stop: None,
            slot_id: None,
        };

        let resp = self
            .provider
            .chat_completion(req)
            .await
            .context("Neural decompilation LLM request failed")?;

        let raw_output = resp.content;
        let rust_code = if let Some(start) = raw_output.find("```rust") {
            let after = &raw_output[start + 7..];
            if let Some(end) = after.find("```") {
                after[..end].trim().to_string()
            } else {
                after.trim().to_string()
            }
        } else {
            raw_output.clone()
        };

        Ok(DecompilationResult {
            function_name: func.name.clone(),
            original_assembly: asm_listing,
            rust_source_code: rust_code,
            explanation: raw_output,
            confidence_score: 0.92,
        })
    }
}
