use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessRefinement {
    pub rule_id: String,
    pub domain: String,
    pub root_cause: String,
    pub injection_rule: String,
}

pub struct HarnessEvolver {
    pub prompt_notes_path: PathBuf,
    pub sglang_endpoint: String,
    pub model_name: String,
}

impl HarnessEvolver {
    pub fn new(prompt_notes_path: PathBuf, sglang_endpoint: &str) -> Self {
        Self {
            prompt_notes_path,
            sglang_endpoint: sglang_endpoint.trim_end_matches('/').to_string(),
            model_name: "Qwen/Qwen3.8-35B-Instruct-AWQ".to_string(),
        }
    }

    pub fn with_model(mut self, model_name: &str) -> Self {
        self.model_name = model_name.to_string();
        self
    }

    /// Diagnoses an oscillation/compiler error trace and appends a prompt note rule to H.ρ
    pub async fn refine_harness(
        &self,
        domain: &str,
        error_fingerprint: &str,
        raw_stderr: &str,
    ) -> Result<HarnessRefinement> {
        let system = r#"
You are the Oxide Continual Harness Evolver.
Diagnose the root cause of the agent failure trace. Formulate a single, concise negative constraint rule to append to the agent's prompt notes (H.ρ).

Output ONLY JSON:
{
  "rule_id": "RULE-xxx",
  "domain": "embedded_rust|pcb_design|cad_3d",
  "root_cause": "Explanation of failure",
  "injection_rule": "NEVER perform X when Y. ALWAYS do Z."
}
"#;

        let user_prompt = format!(
            "Domain: {}\nError Fingerprint: {}\nStderr Trace:\n{}",
            domain, error_fingerprint, raw_stderr
        );

        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model_name,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": 0.1,
            "response_format": { "type": "json_object" }
        });

        let res = client
            .post(format!("{}/v1/chat/completions", self.sglang_endpoint))
            .json(&req_body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid response from SGLang"))?;

        let refinement: HarnessRefinement = serde_json::from_str(content)?;
        self.append_refinement_to_disk(&refinement).await?;

        Ok(refinement)
    }

    pub async fn append_refinement_to_disk(&self, refinement: &HarnessRefinement) -> Result<()> {
        let mut existing = if self.prompt_notes_path.exists() {
            tokio::fs::read_to_string(&self.prompt_notes_path).await?
        } else {
            String::from("# Continual Harness Prompt Notes (H.ρ)\n\n")
        };

        let entry = format!(
            "<!-- REFINE_ID: {} -->\n- **[{}]**: {}\n",
            refinement.rule_id,
            refinement.domain.to_uppercase(),
            refinement.injection_rule
        );

        existing.push_str(&entry);

        if let Some(parent) = self.prompt_notes_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&self.prompt_notes_path, existing).await?;

        tracing::info!(
            "Harness Refinement Applied [{}]: {}",
            refinement.rule_id,
            refinement.injection_rule
        );

        Ok(())
    }

    /// Reverts a rule by its rule_id
    pub async fn revert_refinement(&self, rule_id: &str) -> Result<bool> {
        if !self.prompt_notes_path.exists() {
            return Ok(false);
        }

        let content = tokio::fs::read_to_string(&self.prompt_notes_path).await?;
        let id_marker = format!("<!-- REFINE_ID: {} -->", rule_id);
        
        if !content.contains(&id_marker) {
            return Ok(false);
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();
        let mut skip_next = false;

        for line in lines {
            if line.contains(&id_marker) {
                skip_next = true;
                continue;
            }
            if skip_next {
                skip_next = false;
                continue;
            }
            new_lines.push(line);
        }

        tokio::fs::write(&self.prompt_notes_path, new_lines.join("\n")).await?;
        tracing::info!("Harness Refinement Reverted: {}", rule_id);
        Ok(true)
    }
}
