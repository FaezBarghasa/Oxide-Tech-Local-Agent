use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    pub step_id: usize,
    pub description: String,
    pub assigned_persona: String, // "Architect" | "Coder" | "DRC_Reviewer"
    pub tool_calls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanOutput {
    pub thought: String,
    pub steps: Vec<PlanStep>,
}

pub struct PersonaOrchestrator {
    sglang_client: Client,
    sglang_url: String,
}

impl PersonaOrchestrator {
    pub fn new(sglang_url: &str) -> Self {
        Self {
            sglang_client: Client::new(),
            sglang_url: sglang_url.to_string(),
        }
    }

    pub async fn plan_goal(&self, goal: &str, pruned_context: &str) -> Result<PlanOutput> {
        let prompt = format!(
            "<|im_start|>system\nYou are the Lead Systems Architect. Decompose the request into steps with assigned personas.<|im_end|>\n\
            <|im_start|>user\nContext:\n{}\n\nGoal: {}<|im_end|>\n<|im_start|>assistant\n",
            pruned_context, goal
        );

        let body = serde_json::json!({
            "model": "Qwen3.8-35B-Instruct-AWQ",
            "prompt": prompt,
            "temperature": 0.1,
            "max_tokens": 1024,
            "json_schema": {
                "type": "object",
                "properties": {
                    "thought": { "type": "string" },
                    "steps": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "step_id": { "type": "integer" },
                                "description": { "type": "string" },
                                "assigned_persona": { "type": "string", "enum": ["Researcher", "Architect", "Coder", "DRC_Reviewer"] },
                                "tool_calls": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["step_id", "description", "assigned_persona", "tool_calls"]
                        }
                    }
                },
                "required": ["thought", "steps"]
            }
        });

        let resp = self.sglang_client
            .post(format!("{}/generate", self.sglang_url))
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let raw_text = resp["text"].as_str().unwrap_or_default();
        let plan: PlanOutput = serde_json::from_str(raw_text)?;
        Ok(plan)
    }
}
