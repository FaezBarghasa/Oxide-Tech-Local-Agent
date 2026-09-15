use common::contracts::{InferenceRequest, TaskType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
    /// Spark X 2.5 4B Q8_0 (ultra-lightweight edge/embedded controller reasoning)
    SparkX2_5_4B_Q8_0,
    /// Gemma 4 v2 Q3_K_M (compact memory-constrained local MoE fallback)
    Gemma4_V2_Q3KM,
    /// Gemma 4 Edge 2B Instruction-tuned Q8_0 (high-precision micro-verifier & guardrail gating)
    Gemma4_E2B_IT_Q8_0,
    /// Ornith 1.5 9B Q4_K_M (mid-tier embedded driver & rapid peripheral debugger)
    Ornith1_5_9B_Q4KM,
    /// LLM4Decompile 22B v2 Q6_K (binary analysis, disassembly, decompilation & assembly-to-Rust lifting)
    Llm4Decompile_22B_V2_Q6K,
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
            Self::SparkX2_5_4B_Q8_0 => "Spark-X2.5-4B-Q8_0",
            Self::Gemma4_V2_Q3KM => "gemma4-v2-Q3_K_M",
            Self::Gemma4_E2B_IT_Q8_0 => "gemma-4-e2b-it.Q8_0",
            Self::Ornith1_5_9B_Q4KM => "Ornith-1.5-9B-Q4_K_M",
            Self::Llm4Decompile_22B_V2_Q6K => "llm4decompile-22b-v2.Q6_K",
        }
    }

    pub fn specialization(&self) -> &'static str {
        match self {
            Self::Gemma4_26B_A4B => "MoE Fast Supervisor & Intent Triage (4B active)",
            Self::Qwen3_8_27B => "Generalist Reasoning, Tool Calling & Syntax",
            Self::Ornith1_5_35B_Q4KM => {
                "Deep Architecture, Embedded no_std, Hardware & Verification"
            }
            Self::Qwen3_8_27B_TurboFCFusion => {
                "Heavy Code Generation, Multi-Token Prediction (MTP) & Compiler Fixes"
            }
            Self::SparkX2_5_4B_Q8_0 => "Ultra-low Latency Edge Microcontroller Reasoning",
            Self::Gemma4_V2_Q3KM => "Low-VRAM Compact MoE Budget Execution",
            Self::Gemma4_E2B_IT_Q8_0 => "Edge 2B High-Precision Micro-Verifier & Guardrail Filter",
            Self::Ornith1_5_9B_Q4KM => "Mid-Tier Embedded Driver & Peripheral Debugger",
            Self::Llm4Decompile_22B_V2_Q6K => {
                "Binary Decompilation, Disassembly, ELF & Assembly-to-Rust Lifting"
            }
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
        scores.insert(ExpertModel::SparkX2_5_4B_Q8_0, 0.8);
        scores.insert(ExpertModel::Gemma4_V2_Q3KM, 0.8);
        scores.insert(ExpertModel::Gemma4_E2B_IT_Q8_0, 0.9);
        scores.insert(ExpertModel::Ornith1_5_9B_Q4KM, 0.9);
        scores.insert(ExpertModel::Llm4Decompile_22B_V2_Q6K, 0.8);

        // 1. Task-Type Priors
        match req.task_type {
            TaskType::Architecture | TaskType::Debugging => {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 4.0;
                *scores.get_mut(&ExpertModel::Ornith1_5_9B_Q4KM).unwrap() += 2.5;
            }
            TaskType::CodeCompletion => {
                *scores
                    .get_mut(&ExpertModel::Qwen3_8_27B_TurboFCFusion)
                    .unwrap() += 4.5;
            }
            TaskType::Syntax => {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B).unwrap() += 3.5;
                *scores.get_mut(&ExpertModel::Gemma4_E2B_IT_Q8_0).unwrap() += 2.0;
            }
            TaskType::Training => {
                *scores.get_mut(&ExpertModel::Gemma4_26B_A4B).unwrap() += 3.5;
            }
            TaskType::PcbLayout | TaskType::SceneModeling => {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 3.0;
                *scores.get_mut(&ExpertModel::Ornith1_5_9B_Q4KM).unwrap() += 2.0;
            }
            TaskType::BinaryAnalysis => {
                *scores
                    .get_mut(&ExpertModel::Llm4Decompile_22B_V2_Q6K)
                    .unwrap() += 5.0;
            }
            TaskType::ToolSynthesis => {
                *scores.get_mut(&ExpertModel::Qwen3_8_27B).unwrap() += 4.0;
            }
            TaskType::Research => {
                *scores.get_mut(&ExpertModel::Gemma4_26B_A4B).unwrap() += 4.0;
            }
            TaskType::Verification => {
                *scores.get_mut(&ExpertModel::Gemma4_E2B_IT_Q8_0).unwrap() += 4.5;
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 3.0;
            }
        }

        // 2. Keyword & Domain Signal Gating
        let decompile_signals = [
            "decompile",
            "disassembly",
            "disassemble",
            "objdump",
            "radare2",
            "ghidra",
            "binary",
            "elf",
            "reverse engineer",
            "assembly",
            "asm",
            "symbol table",
            "gdb",
            "hex",
            "stripped",
        ];
        let embedded_heavy_signals = [
            "no_std",
            "firmware",
            "cortex-m",
            "stm32",
            "embassy",
            "probe-rs",
            "embedded-hal",
            "memory barrier",
            "dma",
            "interrupt",
            "redox",
            "driver",
            "bare-metal",
            "rtos",
            "register",
        ];
        let edge_micro_signals = [
            "spark",
            "microcontroller",
            "low-power",
            "pico",
            "avr",
            "tiny",
            "sensor read",
            "gpio toggle",
        ];
        let low_vram_signals = [
            "low-vram",
            "vram budget",
            "q3_k_m",
            "quantized",
            "budget execution",
        ];
        let edge_verifier_signals = [
            "guardrail",
            "filter",
            "sanity check",
            "e2b",
            "pre-pass",
            "quick verify",
        ];
        let code_synthesis_signals = [
            "synthesize",
            "implement",
            "refactor",
            "algorithm",
            "mtp",
            "turbo",
            "optimize function",
            "struct",
            "impl",
            "trait",
            "compiler error",
            "type mismatch",
            "borrowck",
        ];
        let fast_triage_signals = [
            "summarize",
            "classify",
            "explain",
            "fast",
            "triage",
            "overview",
            "diff",
            "checklist",
            "status",
            "moe",
        ];
        let general_signals = [
            "tool", "call", "json", "schema", "parse", "format", "cli", "regex", "search",
        ];

        for sig in decompile_signals {
            if p_lower.contains(sig) {
                *scores
                    .get_mut(&ExpertModel::Llm4Decompile_22B_V2_Q6K)
                    .unwrap() += 4.0;
            }
        }

        for sig in edge_micro_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::SparkX2_5_4B_Q8_0).unwrap() += 3.5;
            }
        }

        for sig in low_vram_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Gemma4_V2_Q3KM).unwrap() += 3.0;
            }
        }

        for sig in edge_verifier_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Gemma4_E2B_IT_Q8_0).unwrap() += 3.0;
            }
        }

        for sig in embedded_heavy_signals {
            if p_lower.contains(sig) {
                *scores.get_mut(&ExpertModel::Ornith1_5_35B_Q4KM).unwrap() += 1.5;
                *scores.get_mut(&ExpertModel::Ornith1_5_9B_Q4KM).unwrap() += 1.0;
            }
        }

        for sig in code_synthesis_signals {
            if p_lower.contains(sig) {
                *scores
                    .get_mut(&ExpertModel::Qwen3_8_27B_TurboFCFusion)
                    .unwrap() += 1.3;
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

/// Operational statistics tracked per expert for adaptive routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertStats {
    pub latency_ema_ms: f32,
    pub quality_ema: f32,
    pub consecutive_failures: u32,
    pub is_circuit_broken: bool,
}

impl Default for ExpertStats {
    fn default() -> Self {
        Self {
            latency_ema_ms: 250.0,
            quality_ema: 1.0,
            consecutive_failures: 0,
            is_circuit_broken: false,
        }
    }
}

/// Adaptive MoE Gating Router with dynamic feedback (latency EMA, quality scoring, circuit breaking).
#[derive(Debug, Clone, Default)]
pub struct AdaptiveMoeGatingRouter {
    inner_router: MoeGatingRouter,
    stats: Arc<RwLock<HashMap<ExpertModel, ExpertStats>>>,
    circuit_breaker_threshold: u32,
}

impl AdaptiveMoeGatingRouter {
    pub fn new() -> Self {
        Self {
            inner_router: MoeGatingRouter::new(),
            stats: Arc::new(RwLock::new(HashMap::new())),
            circuit_breaker_threshold: 3,
        }
    }

    /// Record runtime execution feedback from an expert
    pub fn record_feedback(
        &self,
        expert: ExpertModel,
        latency_ms: f32,
        success: bool,
        quality_score: f32,
    ) {
        if let Ok(mut stats_map) = self.stats.write() {
            let entry = stats_map.entry(expert).or_default();
            // Alpha for EMA = 0.2
            entry.latency_ema_ms = 0.2 * latency_ms + 0.8 * entry.latency_ema_ms;
            entry.quality_ema = 0.2 * quality_score + 0.8 * entry.quality_ema;

            if success {
                entry.consecutive_failures = 0;
                entry.is_circuit_broken = false;
            } else {
                entry.consecutive_failures += 1;
                if entry.consecutive_failures >= self.circuit_breaker_threshold {
                    entry.is_circuit_broken = true;
                }
            }
        }
    }

    /// Route with adaptive adjustments based on health, latency EMA, and quality
    pub fn route(&self, req: &InferenceRequest) -> MoeRoutingDecision {
        let mut decision = self.inner_router.route(req);
        let stats_map = match self.stats.read() {
            Ok(guard) => guard.clone(),
            Err(_) => return decision,
        };

        let mut adjusted_scores: HashMap<ExpertModel, f32> = HashMap::new();
        let all_experts = [
            ExpertModel::Gemma4_26B_A4B,
            ExpertModel::Qwen3_8_27B,
            ExpertModel::Ornith1_5_35B_Q4KM,
            ExpertModel::Qwen3_8_27B_TurboFCFusion,
            ExpertModel::SparkX2_5_4B_Q8_0,
            ExpertModel::Gemma4_V2_Q3KM,
            ExpertModel::Gemma4_E2B_IT_Q8_0,
            ExpertModel::Ornith1_5_9B_Q4KM,
            ExpertModel::Llm4Decompile_22B_V2_Q6K,
        ];

        for expert in all_experts {
            let base_score = decision
                .routing_scores
                .get(expert.model_id())
                .copied()
                .unwrap_or(1.0);

            let stats = stats_map.get(&expert).cloned().unwrap_or_default();

            if stats.is_circuit_broken {
                adjusted_scores.insert(expert, 0.01);
                continue;
            }

            // Latency penalty: reduce score if latency > 500ms
            let latency_factor = (500.0 / stats.latency_ema_ms.max(50.0)).clamp(0.5, 1.5);
            // Quality multiplier (0.5 to 1.5)
            let quality_factor = stats.quality_ema.clamp(0.5, 1.5);

            let final_score = base_score * latency_factor * quality_factor;
            adjusted_scores.insert(expert, final_score);
        }

        let mut sorted: Vec<(ExpertModel, f32)> = adjusted_scores.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let primary_expert = sorted[0].0;
        let secondary_expert = if sorted.len() > 1 && sorted[1].1 > 2.0 {
            Some(sorted[1].0)
        } else {
            None
        };

        let mut routing_scores = HashMap::new();
        for (expert, score) in &sorted {
            routing_scores.insert(expert.model_id().to_string(), *score);
        }

        decision.primary_expert = primary_expert;
        decision.secondary_expert = secondary_expert;
        decision.routing_scores = routing_scores;
        decision.rationale = format!(
            "Adaptive MoE routed to {} (score: {:.2}). Fallback: {:?}",
            primary_expert.model_id(),
            sorted[0].1,
            decision.secondary_expert.map(|e| e.model_id())
        );

        decision
    }
}
