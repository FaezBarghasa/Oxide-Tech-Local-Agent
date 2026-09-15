use common::contracts::{InferenceRequest, TaskType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The target models configured in the Mixture of Experts (MoE) pool
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpertModel {
    /// Google Gemma 4 26B MoE with 4B active parameters (fast triage, supervisor, summarization)
    Gemma4_26B_A4B,
    /// Qwen 3.8 27B generalist model (dialogue, tool calling, syntax parsing)
    Qwen3_8_27B,
    /// Ornith 1.5 35B quantized Q4_K_M (deep systems reasoning, no_std embedded, architecture, verification)
    Ornith1_5_35B_Q4KM,
    /// Qwen 3.8 27B Turbo FCFusion Uncensored Neo-Coder Max MTP Q4_K_M (ultra-fast code synthesis, refactoring, MTP)
    Qwen3_8_27B_TurboFCFusion,
}

impl ExpertModel {
    pub fn model_id(&self) -> &'static str {
        match self {
            Self::Gemma4_26B_A4B => "Gemma-4-26B-A4B",
            Self::Qwen3_8_27B => "qwen3.8-27b",
            Self::Ornith1_5_35B_Q4KM => "Ornith-1.5-35B-Q4_K_M",
            Self::Qwen3_8_27B_TurboFCFusion => {
                "Qwen3.8-27B-TurboFCFusion-735-882-Here-Uncen-NEO-CODER-MAX-MTP-Q4_K_M"
            }
        }
    }

    pub fn specialization(&self) -> &'static str {
        match self {
            Self::Gemma4_26B_A4B => "MoE Fast Supervisor & Intent Triage (4B active)",
            Self::Qwen3_8_27B => "Generalist Reasoning, Tool Calling & Syntax",
            Self::Ornith1_5_35B_Q4KM => "Deep Architecture, Embedded no_std, Hardware & Verification",
            Self::Qwen3_8_27B_TurboFCFusion => "Heavy Code Generation, Multi-Token Prediction (MTP) & Compiler Fixes",
        }
    }
}

/// Routing decision output from the MoE Gating Network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoeRoutingDecision {
    pub primary_expert: ExpertModel,
    pub secondary_expert: Option<ExpertModel>,
    pub routing_scores: HashMap<String, f32>,
    pub rationale: String,
}

/// Mixture of Experts Gating Network
#[derive(Debug, Clone, Default)]
pub struct MoeGatingRouter;

impl MoeGatingRouter {
    pub fn new() -> Self {
        Self
    }

    /// Calculate gating logits and select the best expert(s) for a given request
    pub fn route(&self, req: &InferenceRequest) -> MoeRoutingDecision {
        let p_lower = req.prompt.to_lowercase();
        let mut scores: HashMap<ExpertModel, f32> = HashMap::new();

        scores.insert(ExpertModel::Gemma4_26B_A4B, 1.0);
        scores.insert(ExpertModel::Qwen3_8_27B, 1.2);
        scores.insert(ExpertModel::Ornith1_5_35B_Q4KM, 1.0);
        scores.insert(ExpertModel::Qwen3_8_27B_TurboFCFusion, 1.0);

        // 1. Task-Type Priors
        match req.task_type {
            TaskType::Architecture | TaskType::Debugging => {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 4.0;
            }
            TaskType::CodeCompletion => {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B_TurboFCFusion).unwrap() += 4.5;
            }
            TaskType::Syntax => {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B).unwrap() += 3.5;
            }
            TaskType::Training => {
                *scores.get_mut(&ExpertModel::Gemma4_26B_A4B).unwrap() += 3.5;
            }
            TaskType::PcbLayout | TaskType::SceneModeling => {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 3.0;
            }
        }

        // 2. Keyword & Domain Signal Gating
        let embedded_signals = [
            "no_std", "firmware", "cortex-m", "stm32", "embassy", "probe-rs",
            "embedded-hal", "memory barrier", "dma", "interrupt", "redox",
            "driver", "bare-metal", "rtos", "register",
        ];
        let code_synthesis_signals = [
            "synthesize", "implement", "refactor", "algorithm", "mtp",
            "turbo", "optimize function", "struct", "impl", "trait",
            "compiler error", "type mismatch", "borrowck",
        ];
        let fast_triage_signals = [
            "summarize", "classify", "explain", "fast", "triage", "overview",
            "diff", "checklist", "status", "moe",
        ];
        let general_signals = [
            "tool", "call", "json", "schema", "parse", "format", "cli",
            "regex", "search",
        ];

        for sig in embedded_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 1.5;
            }
        }

        for sig in code_synthesis_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B_TurboFCFusion).unwrap() += 1.3;
            }
        }

        for sig in fast_triage_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Gemma4_26B_A4B).unwrap() += 1.2;
            }
        }

        for sig in general_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B).unwrap() += 1.0;
            }
        }

        // Sort descending by score
        let mut sorted: Vec<(ExpertModel, f32)> = scores.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let primary_expert = sorted[0].0;
        let secondary_expert = if sorted.len() > 1 && sorted[1].1 > 2.5 {
            Some(sorted[1].0)
        } else {
            None
        };

        let mut routing_scores = HashMap::new();
        for (expert, score) in &sorted {
            routing_scores.insert(expert.model_id().to_string(), *score);
        }

        let rationale = format!(
            "Routed to {} (score: {:.2}) for task {:?}. Secondary fallback: {:?}",
            primary_expert.model_id(),
            sorted[0].1,
            req.task_type,
            secondary_expert.map(|e| e.model_id())
        );

        MoeRoutingDecision {
            primary_expert,
            secondary_expert,
            routing_scores,
            rationale,
        }
    }
}
