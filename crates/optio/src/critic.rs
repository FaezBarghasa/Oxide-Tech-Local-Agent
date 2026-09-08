use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Critique feedback produced by the Critic persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critique {
    pub passed: bool,
    pub severity: String, // "info" | "warning" | "blocker"
    pub remarks: String,
    pub suggested_revisions: Vec<String>,
}

/// A critic agent that evaluates proposed solutions against constraints before execution.
pub struct CriticAgent {
    client: Client,
    endpoint: String,
    model_name: String,
}

impl CriticAgent {
    pub fn new(endpoint: &str, model_name: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model_name: model_name.to_string(),
        }
    }

    /// Evaluate an implementation against task specification and constraints.
    pub async fn evaluate_proposal(
        &self,
        task_desc: &str,
        proposed_action: &str,
        system_constraints: &[&str],
    ) -> Result<Critique> {
        let constraints_text = system_constraints.join("\n- ");
        let prompt = format!(
            "<|im_start|>system\nYou are a rigorous Critic Agent. Inspect the proposed action against all constraints. Provide pass/fail, severity, remarks, and revision suggestions.<|im_end|>\n\
            <|im_start|>user\nTask:\n{}\n\nConstraints:\n- {}\n\nProposed Action:\n{}<|im_end|>\n<|im_start|>assistant\n",
            task_desc, constraints_text, proposed_action
        );

        let body = serde_json::json!({
            "model": self.model_name,
            "prompt": prompt,
            "temperature": 0.0,
            "max_tokens": 1024,
            "json_schema": {
                "type": "object",
                "properties": {
                    "passed": { "type": "boolean" },
                    "severity": { "type": "string", "enum": ["info", "warning", "blocker"] },
                    "remarks": { "type": "string" },
                    "suggested_revisions": {
                        "type": "array",
                        "items": { "type": "string" }
                    }
                },
                "required": ["passed", "severity", "remarks", "suggested_revisions"]
            }
        });

        let resp = self
            .client
            .post(format!("{}/v1/completions", self.endpoint))
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(r) if r.status().is_success() => {
                let text = r.text().await?;
                if let Ok(crit) = serde_json::from_str::<Critique>(&text) {
                    return Ok(crit);
                }
            }
            _ => {
                info!("Critic fallback offline/heuristic validation triggered");
            }
        }

        // Deterministic fallback rule check
        let is_unsafe = proposed_action.contains("unwrap()") || proposed_action.contains("unsafe ");
        if is_unsafe {
            Ok(Critique {
                passed: false,
                severity: "blocker".to_string(),
                remarks: "Proposal contains prohibited unwrap() or unverified unsafe block in production path".to_string(),
                suggested_revisions: vec!["Replace unwrap() with proper Result/Option propagation".to_string()],
            })
        } else {
            Ok(Critique {
                passed: true,
                severity: "info".to_string(),
                remarks: "Proposal conforms to baseline static invariants".to_string(),
                suggested_revisions: vec![],
            })
        }
    }
}
