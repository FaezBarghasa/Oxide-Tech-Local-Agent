pub mod delta_harvester;
pub mod graph_optimizer;
pub mod harness_evolver;
pub mod qlora_trainer;
pub mod skill_crystallizer;
pub mod skill_curator;
pub mod tool_maker;

pub use delta_harvester::{DeltaHarvester, VerificationDelta};
pub use graph_optimizer::{GraphTraversalOptimizer, GraphWeightProfile};
pub use harness_evolver::{HarnessEvolver, HarnessRefinement};
pub use qlora_trainer::{QLoraTrainConfig, QLoraTrainer, TrainingReport};
pub use skill_crystallizer::{CrystallizedSkill, SkillCrystallizer};
pub use skill_curator::{SkillCurator, SkillPerformance};
pub use tool_maker::{
    JitToolSpec, MojoToolSpecification, TestResult, ToolMaker, ToolSpecification,
    VerificationResult,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use surrealdb::engine::local::Mem;
    use surrealdb::Surreal;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_tool_synthesis() {
        let tmp = tempdir().unwrap();
        let tools_dir = tmp.path().join("tools");

        let tool_maker = ToolMaker::new(tools_dir.clone(), "http://localhost:8080");

        // Write a test python tool directly and test verification
        tokio::fs::create_dir_all(&tools_dir).await.unwrap();
        let test_py = tools_dir.join("test_tool.py");
        tokio::fs::write(
            &test_py,
            r#"
import sys
import json

def handle_list():
    return {
        "tools": [
            {
                "name": "test_tool",
                "description": "A synthetic test tool",
                "inputSchema": {"type": "object"}
            }
        ]
    }

if __name__ == "__main__":
    print(json.dumps(handle_list()))
"#,
        )
        .await
        .unwrap();

        let res = tool_maker
            .test_generated_tool(&test_py, &serde_json::json!({}))
            .await
            .unwrap();
        assert!(
            res.passed,
            "Sandbox tool test should pass. Stderr: {}",
            res.stderr
        );
        assert!(res.stdout.contains("test_tool"));
    }

    #[tokio::test]
    async fn test_mojo_synthesis() {
        let tmp = tempdir().unwrap();
        let tools_dir = tmp.path().join("mojo_tools");
        tokio::fs::create_dir_all(&tools_dir).await.unwrap();

        let _tool_maker = ToolMaker::new(tools_dir.clone(), "http://localhost:8080");

        let mojo_spec = MojoToolSpecification {
            name: "fast_parser".to_string(),
            description: "SIMD parser".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
            source_code: "fn main():\n    print(\"Mojo Test OK\")\n".to_string(),
        };

        let mojo_file = tools_dir.join(format!("{}.mojo", mojo_spec.name));
        tokio::fs::write(&mojo_file, &mojo_spec.source_code)
            .await
            .unwrap();

        assert!(mojo_file.exists());
        let content = tokio::fs::read_to_string(&mojo_file).await.unwrap();
        assert!(content.contains("Mojo Test OK"));
    }

    #[tokio::test]
    async fn test_skill_mutation() {
        let tmp = tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        tokio::fs::create_dir_all(skills_dir.join("embedded_rust"))
            .await
            .unwrap();

        let skill_file = skills_dir.join("embedded_rust").join("spi_clock_check.md");
        tokio::fs::write(
            &skill_file,
            "# SPI Clock Check Procedure\n\n1. Probe SPI pins\n2. Verify 16MHz clock",
        )
        .await
        .unwrap();

        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db.query("DEFINE TABLE IF NOT EXISTS agent_skill SCHEMALESS;")
            .await
            .unwrap();
        let db_arc = Arc::new(db);

        let curator =
            SkillCurator::new(db_arc.clone(), "http://localhost:8080", skills_dir.clone());

        // Record 2 failures
        curator
            .record_skill_run(
                "spi_clock_check",
                "embedded_rust",
                false,
                Some("Clock timeout on STM32".to_string()),
            )
            .await
            .unwrap();
        curator
            .record_skill_run(
                "spi_clock_check",
                "embedded_rust",
                false,
                Some("Prescaler register mismatch".to_string()),
            )
            .await
            .unwrap();

        // Query performance from SurrealDB
        let mut resp = db_arc
            .query("SELECT * FROM agent_skill WHERE skill_name = $name;")
            .bind(("name", "spi_clock_check".to_string()))
            .await
            .unwrap();
        let perf: Option<SkillPerformance> = resp.take(0).unwrap();
        assert!(perf.is_some());
        let p = perf.unwrap();
        assert_eq!(p.failure_count, 2);
        assert_eq!(p.error_logs.len(), 2);
        assert!(p.error_logs[0].contains("Clock timeout"));
    }

    #[tokio::test]
    async fn test_harness_refine() {
        let tmp = tempdir().unwrap();
        let prompt_notes = tmp.path().join("config").join("prompt_notes.md");

        let evolver = HarnessEvolver::new(prompt_notes.clone(), "http://localhost:8080");

        let refinement = HarnessRefinement {
            rule_id: "RULE-001".to_string(),
            domain: "embedded_rust".to_string(),
            root_cause: "SPI Clock Prescaler mismatch causing transmission timeout".to_string(),
            injection_rule:
                "NEVER configure SPI baud rate > 20MHz without checking APB1 clock divider."
                    .to_string(),
        };

        evolver
            .append_refinement_to_disk(&refinement)
            .await
            .unwrap();

        assert!(prompt_notes.exists());
        let content = tokio::fs::read_to_string(&prompt_notes).await.unwrap();
        assert!(content.contains("<!-- REFINE_ID: RULE-001 -->"));
        assert!(content.contains("NEVER configure SPI baud rate > 20MHz"));

        // Test Revert-by-ID
        let reverted = evolver.revert_refinement("RULE-001").await.unwrap();
        assert!(reverted);
        let reverted_content = tokio::fs::read_to_string(&prompt_notes).await.unwrap();
        assert!(!reverted_content.contains("RULE-001"));
        assert!(!reverted_content.contains("NEVER configure SPI baud rate > 20MHz"));
    }

    #[tokio::test]
    async fn test_graph_traversal_optimization() {
        let mut opt = GraphTraversalOptimizer::new();
        let initial = opt.get_current_profile();
        assert_eq!(initial.max_traversal_hops, 3);

        opt.record_missed_dependency("imports");
        let updated = opt.get_current_profile();
        assert!(updated.dependency_weight > initial.dependency_weight);
        assert_eq!(updated.max_traversal_hops, 4);
    }

    #[tokio::test]
    async fn test_skill_crystallization_and_composition() {
        let tmp = tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        let crystallizer = SkillCrystallizer::new(skills_dir.clone());

        let skill1 = CrystallizedSkill {
            name: "setup_freertos_queue".to_string(),
            description: "Creates FreeRTOS message queue".to_string(),
            tags: vec!["freertos".to_string(), "embedded".to_string()],
            prompt_template: "Setup FreeRTOS queue".to_string(),
            step_sequence: vec![
                "Initialize queue handle".to_string(),
                "Allocate storage buffer".to_string(),
            ],
            source_task_id: "task_01".to_string(),
        };

        let skill_file = crystallizer.crystallize_workflow(&skill1).await.unwrap();
        assert!(skill_file.exists());
        let content = tokio::fs::read_to_string(&skill_file).await.unwrap();
        assert!(content.contains("name: setup_freertos_queue"));
        assert!(content.contains("1. Initialize queue handle"));

        let skill2 = CrystallizedSkill {
            name: "verify_qemu_execution".to_string(),
            description: "Runs bare-metal binary in QEMU".to_string(),
            tags: vec!["qemu".to_string(), "verification".to_string()],
            prompt_template: "Run QEMU".to_string(),
            step_sequence: vec![
                "Launch qemu-system-arm".to_string(),
                "Assert serial output".to_string(),
            ],
            source_task_id: "task_02".to_string(),
        };

        let composite = crystallizer.compose_skills(
            "full_rtos_qemu_pipeline",
            "End to end RTOS verification",
            &[&skill1, &skill2],
        );
        assert_eq!(composite.step_sequence.len(), 6);
        assert!(composite.tags.contains(&"composite".to_string()));
    }

    #[tokio::test]
    async fn test_delta_harvester() {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db.query("DEFINE TABLE IF NOT EXISTS grpo_training_pool SCHEMALESS;")
            .await
            .unwrap();

        let harvester = DeltaHarvester::new(db);
        let prompt = "Implement safe UART buffer";
        let failed_code = "fn send(buf: &[u8]) { let ptr = buf.as_ptr(); }";
        let fixed_code = "fn send(buf: &[u8]) -> Result<(), Error> { if buf.is_empty() { return Ok(()); } Ok(()) }";
        let compiler_err = "error[E0308]: mismatched types";

        let res = harvester
            .record_verified_solution(prompt, failed_code, fixed_code, compiler_err)
            .await;
        assert!(res.is_ok());
    }
}
