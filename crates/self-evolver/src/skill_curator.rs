use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use surrealdb::Connection;
use surrealdb::Surreal;
use surrealdb_types::SurrealValue;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;


#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct SkillPerformance {
    pub skill_name: String,
    pub domain: String,
    pub success_count: u32,
    pub failure_count: u32,
    pub avg_tokens: Option<u32>,
    pub error_logs: Vec<String>,
}


pub struct SkillCurator<C: Connection> {
    pub db: Arc<Surreal<C>>,
    pub vllm_endpoint: String,
    pub model_name: String,
    pub skills_base_dir: PathBuf,
}

impl<C: Connection> SkillCurator<C> {
    pub fn new(db: Arc<Surreal<C>>, vllm_endpoint: &str, skills_base_dir: PathBuf) -> Self {
        Self {
            db,
            vllm_endpoint: vllm_endpoint.trim_end_matches('/').to_string(),
            model_name: "Qwen/Qwen2.5-Coder-32B-Instruct-AWQ".to_string(),
            skills_base_dir,
        }
    }

    pub fn with_model(mut self, model_name: &str) -> Self {
        self.model_name = model_name.to_string();
        self
    }

    /// Record the execution of a skill trajectory in SurrealDB
    pub async fn record_skill_run(
        &self,
        skill_name: &str,
        domain: &str,
        success: bool,
        error_log: Option<String>,
    ) -> Result<()> {
        let _ = self.db.query("DEFINE TABLE IF NOT EXISTS agent_skill SCHEMALESS;").await;

        let sql = "SELECT * FROM agent_skill WHERE skill_name = $name;";
        let mut response = self.db.query(sql).bind(("name", skill_name.to_string())).await?;
        let existing: Option<SkillPerformance> = response.take(0)?;


        let mut perf = existing.unwrap_or_else(|| SkillPerformance {
            skill_name: skill_name.to_string(),
            domain: domain.to_string(),
            success_count: 0,
            failure_count: 0,
            avg_tokens: Some(512),
            error_logs: Vec::new(),
        });

        if success {
            perf.success_count += 1;
        } else {
            perf.failure_count += 1;
            if let Some(err) = error_log {
                perf.error_logs.push(err);
            }
        }

        // Upsert into SurrealDB
        let upsert_sql = r#"
            UPSERT type::record('agent_skill', $name) CONTENT {
                skill_name: $name,
                domain: $domain,
                success_count: $succ,
                failure_count: $fail,
                avg_tokens: $tokens,
                error_logs: $logs
            };
        "#;

        self.db.query(upsert_sql)
            .bind(("name", skill_name.to_string()))
            .bind(("domain", perf.domain.clone()))
            .bind(("succ", perf.success_count))
            .bind(("fail", perf.failure_count))
            .bind(("tokens", perf.avg_tokens))
            .bind(("logs", perf.error_logs.clone()))
            .await?;


        Ok(())
    }

    /// Evaluates a skill's historical performance in SurrealDB and mutates it if degraded (failures >= 2)
    pub async fn evaluate_and_refine_skill(&self, skill_name: &str) -> Result<bool> {
        let sql = "SELECT * FROM agent_skill WHERE skill_name = $name;";
        let mut response = self.db.query(sql).bind(("name", skill_name.to_string())).await?;
        let performance: Option<SkillPerformance> = response.take(0)?;

        if let Some(perf) = performance {
            if perf.failure_count >= 2 && perf.failure_count >= perf.success_count {
                tracing::warn!(
                    "Skill '{}' shows high failure rate ({} fails vs {} passes). Triggering self-mutation...",
                    skill_name,
                    perf.failure_count,
                    perf.success_count
                );

                self.mutate_skill_body(&perf).await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Alias for SkillOpt evaluation and mutation
    pub async fn evaluate_and_mutate_skill(&self, skill_name: &str) -> Result<()> {
        let _ = self.evaluate_and_refine_skill(skill_name).await?;
        Ok(())
    }

    /// Alias for apply_skillopt_edit
    pub async fn apply_skillopt_edit(&self, perf: &SkillPerformance) -> Result<String> {
        self.mutate_skill_body(perf).await
    }


    pub async fn mutate_skill_body(&self, perf: &SkillPerformance) -> Result<String> {
        let skill_file_name = if perf.skill_name.ends_with(".md") {
            perf.skill_name.clone()
        } else {
            format!("{}.md", perf.skill_name)
        };

        let skill_path = self.skills_base_dir
            .join(&perf.domain)
            .join(&skill_file_name);

        let current_body = if skill_path.exists() {
            fs::read_to_string(&skill_path).await?
        } else {
            format!("# Skill: {}\n\nInitial procedure for {}", perf.skill_name, perf.domain)
        };

        let prompt = format!(
            "The following Agent Skill has failed in execution. Analyze the error logs and rewrite the skill procedure to prevent these failures.\n\nCurrent Skill:\n{}\n\nError Logs:\n{}",
            current_body,
            perf.error_logs.join("\n")
        );

        let system = "You are the Oxide Skill Refiner. Return an updated Markdown skill definition that corrects the execution strategy.";
        
        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model_name,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.2
        });

        let res = client.post(format!("{}/v1/chat/completions", self.vllm_endpoint))
            .json(&req_body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let refined_content = json_res["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid response from vLLM during skill mutation"))?;

        if let Some(parent) = skill_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&skill_path, refined_content).await?;
        tracing::info!("Skill '{}' successfully mutated and saved to {:?}", perf.skill_name, skill_path);

        Ok(refined_content.to_string())
    }
}
