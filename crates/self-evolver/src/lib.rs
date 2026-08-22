pub mod tool_maker;
pub mod skill_curator;

pub use tool_maker::{ToolMaker, ToolSpecification, MojoToolSpecification, TestResult};
pub use skill_curator::{SkillCurator, SkillPerformance};

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
        tokio::fs::write(&test_py, r#"
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
"#).await.unwrap();

        let res = tool_maker.test_generated_tool(&test_py, &serde_json::json!({})).await.unwrap();
        assert!(res.passed, "Sandbox tool test should pass. Stderr: {}", res.stderr);
        assert!(res.stdout.contains("test_tool"));
    }

    #[tokio::test]
    async fn test_mojo_synthesis() {
        let tmp = tempdir().unwrap();
        let tools_dir = tmp.path().join("mojo_tools");
        tokio::fs::create_dir_all(&tools_dir).await.unwrap();

        let tool_maker = ToolMaker::new(tools_dir.clone(), "http://localhost:8080");

        let mojo_spec = MojoToolSpecification {
            name: "fast_parser".to_string(),
            description: "SIMD parser".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
            source_code: "fn main():\n    print(\"Mojo Test OK\")\n".to_string(),
        };

        let mojo_file = tools_dir.join(format!("{}.mojo", mojo_spec.name));
        tokio::fs::write(&mojo_file, &mojo_spec.source_code).await.unwrap();

        assert!(mojo_file.exists());
        let content = tokio::fs::read_to_string(&mojo_file).await.unwrap();
        assert!(content.contains("Mojo Test OK"));
    }

    #[tokio::test]
    async fn test_skill_mutation() {
        let tmp = tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        tokio::fs::create_dir_all(skills_dir.join("embedded_rust")).await.unwrap();

        let skill_file = skills_dir.join("embedded_rust").join("spi_clock_check.md");
        tokio::fs::write(&skill_file, "# SPI Clock Check Procedure\n\n1. Probe SPI pins\n2. Verify 16MHz clock").await.unwrap();

        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let db_arc = Arc::new(db);

        let curator = SkillCurator::new(db_arc.clone(), "http://localhost:8080", skills_dir.clone());

        // Record 2 failures
        curator.record_skill_run("spi_clock_check", "embedded_rust", false, Some("Clock timeout on STM32".to_string())).await.unwrap();
        curator.record_skill_run("spi_clock_check", "embedded_rust", false, Some("Prescaler register mismatch".to_string())).await.unwrap();

        // Query performance from SurrealDB
        let mut resp = db_arc.query("SELECT * FROM agent_skill WHERE skill_name = $name;")
            .bind(("name", "spi_clock_check"))
            .await
            .unwrap();
        let perf: Option<SkillPerformance> = resp.take(0).unwrap();
        assert!(perf.is_some());
        let p = perf.unwrap();
        assert_eq!(p.failure_count, 2);
        assert_eq!(p.error_logs.len(), 2);
        assert!(p.error_logs[0].contains("Clock timeout"));
    }
}
