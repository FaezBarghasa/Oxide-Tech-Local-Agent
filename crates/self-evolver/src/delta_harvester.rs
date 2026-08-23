use similar::{ChangeTag, TextDiff};
use serde::{Serialize, Deserialize};
use surrealdb_types::SurrealValue;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct VerificationDelta {
    pub prompt: String,
    pub original_failed_code: String,
    pub verified_fixed_code: String,
    pub compiler_error_log: String,
    pub diff_summary: Vec<String>,
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
        };

        let query = "CREATE grpo_training_pool CONTENT $data;";
        self.db.query(query).bind(("data", record)).await?;
        Ok(())
    }
}
