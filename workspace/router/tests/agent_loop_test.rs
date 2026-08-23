use router::{DynamicLoraRouter, LoraAdapterType, ObserverAgent, LoopRecommendation};
use verifier::CheckpointManager;

#[test]
fn test_dynamic_lora_router_selection() {
    let router = DynamicLoraRouter::new();

    // SQL / DB prompt
    let adapter1 = router.select_adapter_for_prompt("Optimize SurrealQL query for call graph traversal");
    assert_eq!(adapter1, LoraAdapterType::SqlOptimization);

    // Firmware prompt
    let adapter2 = router.select_adapter_for_prompt("Write Embassy #[no_std] STM32F4 UART async driver");
    assert_eq!(adapter2, LoraAdapterType::FirmwareEmbedded);

    // PCB / CAD prompt
    let adapter3 = router.select_adapter_for_prompt("Generate KiCad 8 schematic and netlist for dual H-bridge");
    assert_eq!(adapter3, LoraAdapterType::PcbCad);

    // Security prompt
    let adapter4 = router.select_adapter_for_prompt("Audit Rust buffer memory safety and patch potential CVE");
    assert_eq!(adapter4, LoraAdapterType::SecurityAudit);

    // Generic prompt
    let adapter5 = router.select_adapter_for_prompt("Hello world assistant");
    assert_eq!(adapter5, LoraAdapterType::BaseModel);
}

#[test]
fn test_observer_agent_hallucination_and_loop_pruning() {
    let mut observer = ObserverAgent::new();

    // Normal clean code
    let report1 = observer.evaluate_step(
        1,
        "write_file",
        "use tokio::sync::mpsc;\nuse serde::Deserialize;\npub fn run() {}",
        120,
    );
    assert_eq!(report1.recommendation, LoopRecommendation::Proceed);
    assert!(report1.hallucination_flags.is_empty());

    // Identical step repeated -> loop pruning
    let report2 = observer.evaluate_step(
        2,
        "write_file",
        "use tokio::sync::mpsc;\nuse serde::Deserialize;\npub fn run() {}",
        110,
    );
    assert_eq!(report2.recommendation, LoopRecommendation::PruneNextStep);
    assert!(report2.is_redundant);

    // Hallucinated non-existent crate
    let report3 = observer.evaluate_step(
        3,
        "write_file",
        "use phantom_fake_ai_crate::magic;\nuse super_secret_nonexistent::llm;\nuse imaginary_lib::xyz;",
        95,
    );
    assert_eq!(report3.recommendation, LoopRecommendation::RollbackAndReflect);
    assert_eq!(report3.hallucination_flags.len(), 3);
}

#[tokio::test]
async fn test_checkpoint_manager_snapshot_and_rollback() {
    let cp = CheckpointManager::create_checkpoint("cp_unit_01", "pre-test snapshot", ".")
        .await
        .unwrap();

    assert_eq!(cp.checkpoint_id, "cp_unit_01");

    let rollback_res = CheckpointManager::rollback(&cp, ".").await;
    assert!(rollback_res.is_ok());
}
