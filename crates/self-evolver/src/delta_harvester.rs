use anyhow::Result;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct VerificationDelta {
    pub prompt: String,
    pub original_failed_code: String,
    pub verified_fixed_code: String,
    pub compiler_error_log: String,
    pub diff_summary: Vec<String>,
    pub reward_score: f64,
    pub verification_engine: String,
    pub domain: String,
}

pub struct DeltaHarvester {
    db: Surreal<Db>,
}

impl DeltaHarvester {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }

    pub async fn record_verified_solution(
        &self,
        prompt: &str,
        failed_code: &str,
        fixed_code: &str,
        compiler_log: &str,
    ) -> Result<()> {
        self.record_verified_solution_with_reward(
            prompt,
            failed_code,
            fixed_code,
            compiler_log,
            1.0,
            "cargo_check",
            "embedded_rust",
        )
        .await
    }

    pub async fn record_verified_solution_with_reward(
        &self,
        prompt: &str,
        failed_code: &str,
        fixed_code: &str,
        compiler_log: &str,
        reward_score: f64,
        verification_engine: &str,
        domain: &str,
    ) -> Result<()> {
        let diff = TextDiff::from_lines(failed_code, fixed_code);
        let mut diff_summary = Vec::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            if change.tag() != ChangeTag::Equal {
                diff_summary.push(format!("{}{}", sign, change));
            }
        }

        let record = VerificationDelta {
            prompt: prompt.to_string(),
            original_failed_code: failed_code.to_string(),
            verified_fixed_code: fixed_code.to_string(),
            compiler_error_log: compiler_log.to_string(),
            diff_summary,
            reward_score,
            verification_engine: verification_engine.to_string(),
            domain: domain.to_string(),
        };

        let query = "CREATE grpo_training_pool CONTENT $data;";
        self.db.query(query).bind(("data", record)).await?;
        Ok(())
    }
}
