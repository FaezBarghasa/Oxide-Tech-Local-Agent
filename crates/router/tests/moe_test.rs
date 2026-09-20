use common::contracts::{ContextSnapshot, InferenceRequest, TaskType};
use router::moe_router::{ExpertModel, MoeGatingRouter};
use std::collections::HashMap;

fn mock_request(task_type: TaskType, prompt: &str) -> InferenceRequest {
    InferenceRequest {
        task_type,
        prompt: prompt.to_string(),
        context: ContextSnapshot {
            files: HashMap::new(),
            ast_summary: None,
            board_state: None,
        },
        tenant: "default".to_string(),
        local_failures: 0,
    }
}

#[test]
fn test_moe_routing_ornith_embedded_and_architecture() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::Architecture,
        "Design an async Embassy DMA SPI driver for STM32F407 bare-metal no_std firmware",
    );
    let decision = router.route(&req);
    assert_eq!(decision.primary_expert, ExpertModel::Ornith1_5_35B_Q4KM);
    assert_eq!(decision.primary_expert.model_id(), "Ornith-1.5-35B-Q4_K_M");
}

#[test]
fn test_moe_routing_turbo_fc_fusion_code_completion() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::CodeCompletion,
        "Synthesize high-performance MTP algorithm refactoring and optimize trait impl",
    );
    let decision = router.route(&req);
    assert_eq!(
        decision.primary_expert,
        ExpertModel::Qwen3_8_27B_TurboFCFusion
    );
    assert_eq!(
        decision.primary_expert.model_id(),
        "Qwen3.8-27B-TurboFCFusion-735-882-Here-Uncen-NEO-CODER-MAX-MTP-Q4_K_M"
    );
}

#[test]
fn test_moe_routing_qwen_generalist_syntax() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::Syntax,
        "Validate JSON schema and parse CLI tool parameters regex",
    );
    let decision = router.route(&req);
    assert_eq!(decision.primary_expert, ExpertModel::Qwen3_8_27B);
    assert_eq!(decision.primary_expert.model_id(), "qwen3.8-27b");
}

#[test]
fn test_moe_routing_gemma_moe_fast_triage_and_training() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::Training,
        "Summarize and triage agent loop trace dataset with fast MoE overview",
    );
    let decision = router.route(&req);
    assert_eq!(decision.primary_expert, ExpertModel::Gemma4_26B_A4B);
    assert_eq!(decision.primary_expert.model_id(), "Gemma-4-26B-A4B");
}

#[test]
fn test_moe_routing_llm4decompile_binary_analysis() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::Debugging,
        "Decompile stripped ELF binary and disassemble x86_64 assembly function to Rust",
    );
    let decision = router.route(&req);
    assert_eq!(
        decision.primary_expert,
        ExpertModel::Llm4Decompile_22B_V2_Q6K
    );
    assert_eq!(
        decision.primary_expert.model_id(),
        "llm4decompile-22b-v2.Q6_K"
    );
}

#[test]
fn test_moe_routing_spark_edge_microcontroller() {
    let router = MoeGatingRouter::new();

    let req = mock_request(
        TaskType::CodeCompletion,
        "Spark ultra-low power microcontroller sensor read and gpio toggle",
    );
    let decision = router.route(&req);
    assert_eq!(decision.primary_expert, ExpertModel::SparkX2_5_4B_Q8_0);
    assert_eq!(decision.primary_expert.model_id(), "Spark-X2.5-4B-Q8_0");
}

#[test]
fn test_moe_routing_all_nine_models_and_specializations() {
    let models = [
        (ExpertModel::Gemma4_26B_A4B, "Gemma-4-26B-A4B"),
        (ExpertModel::Qwen3_8_27B, "qwen3.8-27b"),
        (ExpertModel::Ornith1_5_35B_Q4KM, "Ornith-1.5-35B-Q4_K_M"),
        (
            ExpertModel::Qwen3_8_27B_TurboFCFusion,
            "Qwen3.8-27B-TurboFCFusion-735-882-Here-Uncen-NEO-CODER-MAX-MTP-Q4_K_M",
        ),
        (ExpertModel::SparkX2_5_4B_Q8_0, "Spark-X2.5-4B-Q8_0"),
        (ExpertModel::Gemma4_V2_Q3KM, "gemma4-v2-Q3_K_M"),
        (ExpertModel::Gemma4_E2B_IT_Q8_0, "gemma-4-e2b-it.Q8_0"),
        (ExpertModel::Ornith1_5_9B_Q4KM, "Ornith-1.5-9B-Q4_K_M"),
        (
            ExpertModel::Llm4Decompile_22B_V2_Q6K,
            "llm4decompile-22b-v2.Q6_K",
        ),
    ];

    for (model, expected_id) in models {
        assert_eq!(model.model_id(), expected_id);
        assert!(!model.specialization().is_empty());
    }
}
