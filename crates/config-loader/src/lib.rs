use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Cannot read config file at '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("TOML parse error in '{path}': {source}")]
    Parse {
        path: String,
        #[source]
        source: toml::de::Error,
    },
}

// ── Top-level config ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub gateway: GatewayConfig,
    pub thinker: ModelConfig,
    pub coder: CoderConfig,
    pub rag: RagConfig,
    pub mcp: McpConfig,
    #[serde(default)]
    pub perception: PerceptionConfig,
}

// ── External Research & Perception Layer ─────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct PerceptionConfig {
    pub default_engine: String, // "scrapling" | "pinchtab" | "kitesurf" | "auto"
    pub auto_escalate_on_block: bool,
    pub max_parallel_research_tasks: usize,
    pub grounding_confidence_threshold: f32,
    pub scrapling: ScraplingConfig,
    pub pinchtab: PinchTabConfig,
    pub kitesurf: KitesurfConfig,
}

impl Default for PerceptionConfig {
    fn default() -> Self {
        Self {
            default_engine: "auto".to_string(),
            auto_escalate_on_block: true,
            max_parallel_research_tasks: 8,
            grounding_confidence_threshold: 0.80,
            scrapling: ScraplingConfig::default(),
            pinchtab: PinchTabConfig::default(),
            kitesurf: KitesurfConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScraplingConfig {
    pub stealth_mode: bool,
    pub user_agent_rotation: bool,
    pub request_timeout_secs: u64,
    pub max_retries: usize,
}

impl Default for ScraplingConfig {
    fn default() -> Self {
        Self {
            stealth_mode: true,
            user_agent_rotation: true,
            request_timeout_secs: 15,
            max_retries: 3,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PinchTabConfig {
    pub endpoint: String,
    pub port: u16,
    pub cloak_mode: bool,
    pub auto_start: bool,
    pub timeout_secs: u64,
}

impl Default for PinchTabConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:9876".to_string(),
            port: 9876,
            cloak_mode: true,
            auto_start: false,
            timeout_secs: 30,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KitesurfConfig {
    pub account_id_env: String,
    pub api_token_env: String,
    pub endpoint: String,
    pub isolate_timeout_ms: u64,
    pub enable_screenshot_verification: bool,
    pub enable_pdf_extraction: bool,
}

impl Default for KitesurfConfig {
    fn default() -> Self {
        Self {
            account_id_env: "CLOUDFLARE_ACCOUNT_ID".to_string(),
            api_token_env: "CLOUDFLARE_API_TOKEN".to_string(),
            endpoint: "https://api.cloudflare.com/client/v4/accounts/{account_id}/browser-rendering".to_string(),
            isolate_timeout_ms: 30000,
            enable_screenshot_verification: true,
            enable_pdf_extraction: true,
        }
    }
}

// ── Gateway / router ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    /// Latency threshold in milliseconds.  If the online API RTT exceeds this,
    /// the router falls back to the local coder.
    pub latency_threshold_ms: u64,
    /// `cargo check` quality score [0.0 – 1.0] below which the current backend
    /// is marked "unsatisfying" and the alternate backend is tried.
    pub quality_threshold: f32,
    /// Maximum auto-healing retries before returning FAILED.
    pub max_retries: usize,
}

// ── Model / LLM endpoint ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    /// `"ollama"` | `"groq"` | `"mistral"` | `"vllm"`
    pub provider: String,
    pub model: String,
    pub base_url: String,
}

// ── Coder section ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct CoderConfig {
    pub online: OnlineCoderConfig,
    pub local: ModelConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OnlineCoderConfig {
    pub primary: ModelConfig,
    pub secondary: ModelConfig,
}

// ── RAG pipeline ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct RagConfig {
    pub collection: String,
    pub top_k: usize,
    pub auto_update_on_startup: bool,
    pub watchlist: Vec<String>,
}

// ── MCP server ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct McpConfig {
    /// `"stdio"` or `"tcp"`
    pub transport: String,
    pub tcp_port: u16,
}

// ── Loader ────────────────────────────────────────────────────────────────────

impl AppConfig {
    /// Load from the given TOML file path.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let raw = std::fs::read_to_string(path.as_ref()).map_err(|e| ConfigError::Io {
            path: path_str.clone(),
            source: e,
        })?;
        toml::from_str(&raw).map_err(|e| ConfigError::Parse {
            path: path_str,
            source: e,
        })
    }

    /// Convenience: load `config.toml` relative to the workspace root, then
    /// fall back to the current directory.  Returns an error only if the file
    /// exists but cannot be parsed; if the file is missing entirely a default
    /// in-memory config is returned so the server still starts.
    pub fn load_default() -> Result<Self, ConfigError> {
        let candidates = [
            // Workspace root (when running `cargo run` from the project dir).
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../config.toml"),
            "config.toml",
        ];

        for candidate in &candidates {
            let p = Path::new(candidate);
            if p.exists() {
                return Self::from_file(p);
            }
        }

        // No config.toml found — return hard-coded defaults so the binary
        // still boots in development without a config file present.
        Ok(Self::default_config())
    }

    fn default_config() -> Self {
        AppConfig {
            gateway: GatewayConfig {
                latency_threshold_ms: 2000,
                quality_threshold: 0.85,
                max_retries: 5,
            },
            thinker: ModelConfig {
                provider: "ollama".to_string(),
                model: "qwen2.5-coder:32b-instruct-q4_K_M".to_string(),
                base_url: "http://localhost:11434".to_string(),
            },
            coder: CoderConfig {
                online: OnlineCoderConfig {
                    primary: ModelConfig {
                        provider: "groq".to_string(),
                        model: "llama-3.3-70b-versatile".to_string(),
                        base_url: "https://api.groq.com/openai/v1".to_string(),
                    },
                    secondary: ModelConfig {
                        provider: "mistral".to_string(),
                        model: "codestral-latest".to_string(),
                        base_url: "https://api.mistral.ai/v1".to_string(),
                    },
                },
                local: ModelConfig {
                    provider: "ollama".to_string(),
                    model: "qwen2.5-coder:7b-instruct-q4_K_M".to_string(),
                    base_url: "http://localhost:11434".to_string(),
                },
            },
            rag: RagConfig {
                collection: "rust_rag".to_string(),
                top_k: 8,
                auto_update_on_startup: true,
                watchlist: vec![
                    "embedded-hal".to_string(),
                    "embassy-executor".to_string(),
                    "rtic".to_string(),
                    "cortex-m".to_string(),
                    "defmt".to_string(),
                    "heapless".to_string(),
                ],
            },
            mcp: McpConfig {
                transport: "stdio".to_string(),
                tcp_port: 9090,
            },
            perception: PerceptionConfig::default(),
        }
    }
}
