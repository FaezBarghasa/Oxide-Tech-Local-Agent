use common::config::AppConfig;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Domain {
    Embedded,
    Backend,
    PCB,
    CAD,
    Simulation,
    Documentation,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    Generate,
    Refactor,
    Review,
    Design,
    Upgrade,
    Analyze,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThinkerOutput {
    pub domain: Domain,
    pub skill: SkillType,
    pub plan: Vec<String>,
    pub verification: Vec<String>,
    pub coder_prompt: String,
}

#[derive(Default)]
pub struct ThinkerClient;

const THINKER_SYSTEM_PROMPT: &str = r#"
You are the Chief Architecture Thinker for Oxide-Tech. Your job is to classify the task, design a step-by-step implementation plan, identify verification checks, and form a detailed coder prompt. You never write the implementation code yourself.

You MUST respond ONLY with a single valid JSON object. Do not include markdown code fences, formatting, or explanations outside the JSON object. The JSON object must strictly match the following schema:
{
  "domain": "embedded" | "backend" | "pcb" | "cad" | "simulation" | "documentation",
  "skill": "generate" | "refactor" | "review" | "design" | "upgrade" | "analyze",
  "plan": ["step 1", "step 2", ...],
  "verification": ["check 1", "check 2", ...],
  "coder_prompt": "A self-contained detailed prompt instructing a coder model how to implement the mandate, including details about libraries, files, API calls, and logic."
}
"#;

impl ThinkerClient {
    pub fn new() -> Self {
        Self
    }

    /// Run a generic LLM completion query using the configured thinker model.
    pub async fn complete_prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        config: &AppConfig,
    ) -> Result<String, anyhow::Error> {
        complete_llm(config, system_prompt, user_prompt).await
    }

    /// Analyse the task using the configured Thinker LLM and produce a detailed plan and coder prompt.
    pub async fn plan_task(
        &self,
        prompt: &str,
        config: &AppConfig,
    ) -> Result<ThinkerOutput, anyhow::Error> {
        info!(
            provider = %config.thinker.provider,
            model = %config.thinker.model,
            "Thinker planning task using LLM"
        );

        let user_prompt = format!(
            "Mandate:\n{}\n\nGenerate the structured architectural ThinkerOutput JSON.",
            prompt
        );

        let raw_resp = complete_llm(config, THINKER_SYSTEM_PROMPT, &user_prompt).await?;
        let cleaned = clean_json_response(&raw_resp);

        let output: ThinkerOutput = serde_json::from_str(&cleaned).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse Thinker output JSON: {}. Raw response: {}",
                e,
                raw_resp
            )
        })?;

        Ok(output)
    }
}

fn clean_json_response(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("```json").unwrap_or(s);
    let s = s.strip_prefix("```").unwrap_or(s);
    let s = s.strip_suffix("```").unwrap_or(s);
    s.trim().to_string()
}

async fn complete_llm(
    config: &AppConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, anyhow::Error> {
    let provider = config.thinker.provider.to_lowercase();
    let model = &config.thinker.model;
    let base_url = config.thinker.base_url.trim_end_matches('/');

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    if provider == "ollama" {
        let url = format!("{}/api/chat", base_url);
        let body = serde_json::json!({
            "model": model,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "stream": false,
            "options": {
                "temperature": 0.1
            }
        });

        let resp = client.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Ollama error ({}): {}", status, err_text);
        }

        let parsed: serde_json::Value = resp.json().await?;
        let content = parsed
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| anyhow::anyhow!("Ollama returned empty response"))?;

        Ok(content.to_string())
    } else {
        // OpenAI compatible chat completions
        let url = format!("{}/chat/completions", base_url);

        let api_key = match provider.as_str() {
            "google" | "gemini" => std::env::var("GEMINI_API_KEY")
                .or_else(|_| std::env::var("GOOGLE_API_KEY"))
                .ok(),
            "groq" => std::env::var("GROQ_API_KEY").ok(),
            "mistral" | "codestral" => std::env::var("MISTRAL_API_KEY").ok(),
            "deepseek" => std::env::var("DEEPSEEK_API_KEY").ok(),
            "cerebras" => std::env::var("CEREBRAS_API_KEY").ok(),
            "openrouter" => std::env::var("OPENROUTER_API_KEY").ok(),
            "nvidia" | "nvidia-nim" => std::env::var("NVIDIA_API_KEY").ok(),
            "sambanova" => std::env::var("SAMBANOVA_API_KEY").ok(),
            "cohere" => std::env::var("COHERE_API_KEY").ok(),
            "agnes" | "agnes-ai" => std::env::var("AGNES_API_KEY").ok(),
            _ => std::env::var("OPENAI_API_KEY").ok(),
        };

        let body = serde_json::json!({
            "model": model,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "temperature": 0.1,
            "response_format": { "type": "json_object" }
        });

        let mut req = client.post(&url).json(&body);
        if let Some(key) = api_key {
            req = req.bearer_auth(key);
        }

        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("API provider error ({}): {}", status, err_text);
        }

        let parsed: serde_json::Value = resp.json().await?;
        let content = parsed
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|a| a.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| anyhow::anyhow!("API returned empty choices"))?;

        Ok(content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_json_response() {
        let input_with_fences = "```json\n{\n  \"domain\": \"embedded\"\n}\n```";
        let cleaned = clean_json_response(input_with_fences);
        assert_eq!(cleaned, "{\n  \"domain\": \"embedded\"\n}");

        let input_no_fences = "{\n  \"domain\": \"embedded\"\n}";
        let cleaned = clean_json_response(input_no_fences);
        assert_eq!(cleaned, "{\n  \"domain\": \"embedded\"\n}");
    }

    #[test]
    fn test_domain_serialization() {
        let output = ThinkerOutput {
            domain: Domain::Embedded,
            skill: SkillType::Generate,
            plan: vec!["setup".to_string()],
            verification: vec!["check".to_string()],
            coder_prompt: "do it".to_string(),
        };

        let serialized = serde_json::to_string(&output).unwrap();
        assert!(serialized.contains("\"domain\":\"embedded\""));
        assert!(serialized.contains("\"skill\":\"generate\""));

        let deserialized: ThinkerOutput = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.domain, Domain::Embedded);
        assert_eq!(deserialized.skill, SkillType::Generate);
    }
}
