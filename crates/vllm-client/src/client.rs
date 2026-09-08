use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tracing::{debug, warn};

use config_loader::ModelConfig;

// ── Provider enum ────────────────────────────────────────────────────────────

/// The backing inference provider for a `LlmRouterClient` instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmProvider {
    /// Self-hosted vLLM instance — OpenAI-compatible endpoint.
    Vllm,
    /// Groq cloud — OpenAI-compatible endpoint (free tier).
    Groq,
    /// Mistral Le Chat — OpenAI-compatible endpoint (free / codestral tier).
    MistralLeChat,
    /// Google AI Studio / Gemini API — OpenAI-compatible shim.
    /// Models: `gemma-4-27b-it`, `gemini-2.0-flash`, etc.
    /// Requires `GEMINI_API_KEY` in the environment.
    Google,
    /// Local Ollama — uses the `/api/chat` endpoint (JSON stream).
    Ollama,
    /// Cerebras — OpenAI-compatible endpoint (free tier).
    Cerebras,
    /// OpenRouter — OpenAI-compatible endpoint (free tier).
    OpenRouter,
    /// DeepSeek — OpenAI-compatible endpoint.
    DeepSeek,
    /// GitHub Models — OpenAI-compatible endpoint (free tier).
    GitHubModels,
    /// SambaNova — OpenAI-compatible endpoint (free tier).
    SambaNova,
    /// NVIDIA NIM — OpenAI-compatible endpoint (free tier).
    Nvidia,
    /// Cloudflare Workers AI — OpenAI-compatible endpoint (free tier).
    Cloudflare,
    /// Cohere — OpenAI-compatible endpoint.
    Cohere,
    /// Pollinations AI — OpenAI-compatible endpoint, no key required.
    Pollinations,
    /// Zhipu (Z.ai GLM) — OpenAI-compatible endpoint (free tier).
    Zhipu,
    /// Agnes AI — OpenAI-compatible endpoint.
    Agnes,
}

impl LlmProvider {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "groq" => Self::Groq,
            "mistral" => Self::MistralLeChat,
            "google" | "gemini" | "gemma" => Self::Google,
            "ollama" => Self::Ollama,
            "cerebras" => Self::Cerebras,
            "openrouter" => Self::OpenRouter,
            "deepseek" => Self::DeepSeek,
            "github" | "github-models" | "githubmodels" => Self::GitHubModels,
            "sambanova" => Self::SambaNova,
            "nvidia" | "nvidia-nim" => Self::Nvidia,
            "cloudflare" | "cloudflare-workers" => Self::Cloudflare,
            "cohere" => Self::Cohere,
            "pollinations" | "pollinations-ai" => Self::Pollinations,
            "zhipu" | "zhipuai" | "z.ai" => Self::Zhipu,
            "agnes" | "agnes-ai" | "agnesai" => Self::Agnes,
            _ => Self::Vllm,
        }
    }
}

// ── OpenAI-compatible message / request / response ───────────────────────────

#[derive(Debug, Serialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Clone)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Debug, Serialize, Clone)]
struct OpenAiRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
    // Disable streaming — we read the full response body.
    stream: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct OpenAiChoiceMessage {
    content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct OpenAiChoice {
    message: OpenAiChoiceMessage,
}

#[derive(Debug, Deserialize, Clone)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

// ── Ollama-specific structs ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Clone)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize, Clone)]
struct OllamaOptions {
    temperature: f32,
    num_predict: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct OllamaResponse {
    message: OllamaMessage,
}

// ── Main client ───────────────────────────────────────────────────────────────

/// A unified LLM client that dispatches to Groq, Mistral, Ollama, or a
/// self-hosted vLLM instance based on the configured provider.
///
/// API keys are resolved from environment variables:
/// - `GROQ_API_KEY`
/// - `MISTRAL_API_KEY`
/// - `VLLM_API_KEY` (optional, for authenticated vLLM deployments)
///
/// Ollama never requires a key.
pub struct LlmRouterClient {
    provider: LlmProvider,
    model: String,
    base_url: String,
    http: Client,
}

impl LlmRouterClient {
    /// Construct from a `ModelConfig` slice (parsed from `config.toml`).
    pub fn from_config(cfg: &ModelConfig) -> Self {
        let provider = LlmProvider::from_str(&cfg.provider);
        Self {
            provider,
            model: cfg.model.clone(),
            base_url: cfg.base_url.trim_end_matches('/').to_string(),
            http: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("Failed to build reqwest client"),
        }
    }

    /// Build an explicit Ollama client (convenience constructor).
    pub fn ollama(base_url: &str, model: &str) -> Self {
        Self {
            provider: LlmProvider::Ollama,
            model: model.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            http: Client::builder()
                .timeout(Duration::from_secs(300))
                .build()
                .expect("Failed to build reqwest client"),
        }
    }

    /// Send a system + user prompt and return the model's text completion.
    ///
    /// `expect_json` instructs OpenAI-compatible backends to use
    /// `response_format: { type: "json_object" }`.  Ollama ignores this flag
    /// (JSON mode is controlled via the system prompt instead).
    #[tracing::instrument(name = "llm_complete", skip(self, system_prompt, user_prompt), fields(provider = ?self.provider, model = %self.model, expect_json = expect_json))]
    pub async fn complete(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        expect_json: bool,
    ) -> Result<String, anyhow::Error> {
        match self.provider {
            LlmProvider::Ollama => self.complete_ollama(system_prompt, user_prompt).await,
            // Groq, Mistral, Google (Gemini shim), and vLLM all speak
            // OpenAI-compatible /chat/completions.
            _ => {
                self.complete_openai_compatible(system_prompt, user_prompt, expect_json)
                    .await
            }
        }
    }

    // ── OpenAI-compatible path (Groq, Mistral, vLLM) ────────────────────────

    #[tracing::instrument(name = "llm_openai_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
    async fn complete_openai_compatible(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        expect_json: bool,
    ) -> Result<String, anyhow::Error> {
        let url = format!("{}/chat/completions", self.base_url);

        let api_key = self.resolve_api_key();

        let response_format = if expect_json {
            Some(ResponseFormat {
                r#type: "json_object".to_string(),
            })
        } else {
            None
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.05,
            response_format,
            stream: false,
        };

        debug!(provider = ?self.provider, model = %self.model, "Sending OpenAI-compatible request");

        let mut req = self.http.post(&url).json(&body);
        if let Some(key) = api_key {
            req = req.bearer_auth(key);
        }

        let resp = req.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("LLM API error {}: {}", status, text);
        }

        let parsed: OpenAiResponse = resp.json().await?;

        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| anyhow::anyhow!("LLM returned empty choices"))
    }

    /// Resolve the bearer token for the current provider from env vars.
    fn resolve_api_key(&self) -> Option<String> {
        match self.provider {
            LlmProvider::Groq => env::var("GROQ_API_KEY").ok(),
            LlmProvider::MistralLeChat => env::var("MISTRAL_API_KEY").ok(),
            LlmProvider::Google => {
                // Google AI Studio uses x-goog-api-key header, but the
                // OpenAI-compatible shim also accepts a Bearer token.
                env::var("GEMINI_API_KEY")
                    .or_else(|_| env::var("GOOGLE_API_KEY"))
                    .ok()
            }
            LlmProvider::Vllm => env::var("VLLM_API_KEY").ok(),
            LlmProvider::Ollama => None,
            LlmProvider::Cerebras => env::var("CEREBRAS_API_KEY").ok(),
            LlmProvider::OpenRouter => env::var("OPENROUTER_API_KEY").ok(),
            LlmProvider::DeepSeek => env::var("DEEPSEEK_API_KEY").ok(),
            LlmProvider::GitHubModels => env::var("GITHUB_TOKEN")
                .or_else(|_| env::var("GITHUB_API_KEY"))
                .ok(),
            LlmProvider::SambaNova => env::var("SAMBANOVA_API_KEY").ok(),
            LlmProvider::Nvidia => env::var("NVIDIA_API_KEY").ok(),
            LlmProvider::Cloudflare => env::var("CLOUDFLARE_API_KEY")
                .or_else(|_| env::var("CLOUDFLARE_API_TOKEN"))
                .ok(),
            LlmProvider::Cohere => env::var("COHERE_API_KEY").ok(),
            LlmProvider::Pollinations => None,
            LlmProvider::Zhipu => env::var("ZHIPU_API_KEY")
                .or_else(|_| env::var("ZHIPUAI_API_KEY"))
                .ok(),
            LlmProvider::Agnes => env::var("AGNES_API_KEY").ok(),
        }
    }

    // ── Ollama path ──────────────────────────────────────────────────────────

    #[tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
    async fn complete_ollama(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let url = format!("{}/api/chat", self.base_url);

        let body = OllamaRequest {
            model: self.model.clone(),
            messages: vec![
                OllamaMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OllamaMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            stream: false,
            options: OllamaOptions {
                temperature: 0.05,
                num_predict: 4096,
            },
        };

        debug!(model = %self.model, "Sending Ollama request");

        let resp = self.http.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Ollama API error {}: {}", status, text);
        }

        let parsed: OllamaResponse = resp.json().await?;
        Ok(parsed.message.content)
    }

    /// Emit a diagnostic warning when the current backend fails, visible in
    /// structured tracing logs.
    pub fn warn_backend_failure(&self, reason: &str) {
        warn!(
            provider = ?self.provider,
            model = %self.model,
            reason = reason,
            "LLM backend failed"
        );
    }
}
