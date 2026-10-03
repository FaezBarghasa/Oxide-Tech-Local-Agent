use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ModelRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub model_id: String,
    pub canonical_name: String,
    pub provider: String,
    pub context_window: u32,
    pub supports_thinking: bool,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub is_free_tier: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ComboStep {
    pub provider: String,
    pub model: String,
    pub account_id: Option<String>,
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ComboRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub name: String,
    pub strategy: String,
    pub targets: Vec<ComboStep>,
    pub handoff_threshold_percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct AccountRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub account_id: String,
    pub provider: String,
    pub auth_type: String, // "api_key", "session_cookie", "gecko_profile"
    pub credentials_encrypted: Vec<u8>,
    pub rpm_limit: u32,
    pub rpd_limit: u32,
    pub tokens_used_today: u64,
    pub consecutive_429_count: u32,
    pub cool_down_until_ms: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct SessionRelayRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub session_id: String,
    pub handoff_summary: String,
    pub token_count: usize,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct GatewayDb {
    pub db: Surreal<Any>,
}

impl GatewayDb {
    pub async fn init<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let db_path = data_dir.as_ref().join("gateway_store");
        let _ = tokio::fs::create_dir_all(&db_path).await;

        let db_url = std::env::var("OXIDE_GATEWAY_DB_URL")
            .unwrap_or_else(|_| format!("surrealkv://{}", db_path.display()));

        let db = match connect(&db_url).await {
            Ok(d) => d,
            Err(e) => {
                tracing::warn!(
                    "Failed to open db at {}: {}. Falling back to mem://",
                    db_url,
                    e
                );
                connect("mem://")
                    .await
                    .context("Failed to open mem:// SurrealDB instance")?
            }
        };

        db.use_ns("oxide").use_db("gateway").await?;

        let instance = Self { db };
        instance.seed_default_combos().await?;
        instance.seed_model_catalog().await?;

        Ok(instance)
    }

    pub async fn init_mem() -> Result<Self> {
        let db = connect("mem://")
            .await
            .context("Failed to open mem:// SurrealDB instance")?;
        db.use_ns("oxide").use_db("gateway").await?;
        let instance = Self { db };
        instance.seed_default_combos().await?;
        instance.seed_model_catalog().await?;
        Ok(instance)
    }

    pub async fn get_combo(&self, name: &str) -> Result<Option<ComboRecord>> {
        let mut res = self
            .db
            .query("SELECT * FROM combo WHERE name = $name")
            .bind(("name", name.to_string()))
            .await?;
        let mut combos: Vec<ComboRecord> = res.take(0).unwrap_or_default();
        Ok(combos.pop())
    }

    pub async fn list_combos(&self) -> Result<Vec<ComboRecord>> {
        let mut res = self.db.query("SELECT * FROM combo").await?;
        let combos: Vec<ComboRecord> = res.take(0).unwrap_or_default();
        Ok(combos)
    }

    pub async fn get_healthy_account(&self, provider: &str) -> Result<Option<AccountRecord>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;

        let mut res = self
            .db
            .query(
                r#"
                SELECT * FROM account 
                WHERE provider = $provider 
                  AND is_active = true 
                  AND cool_down_until_ms <= $now
                ORDER BY tokens_used_today ASC
                LIMIT 1
            "#,
            )
            .bind(("provider", provider.to_string()))
            .bind(("now", now as i64))
            .await?;

        let mut accounts: Vec<AccountRecord> = res.take(0).unwrap_or_default();
        Ok(accounts.pop())
    }

    pub async fn report_429(&self, account_id: &str, backoff_secs: u64) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;
        let cooldown_until = now + (backoff_secs * 1000);

        self.db
            .query(
                r#"
                UPDATE account SET 
                    consecutive_429_count = consecutive_429_count + 1,
                    cool_down_until_ms = $cooldown
                WHERE id = $id
            "#,
            )
            .bind(("id", account_id.to_string()))
            .bind(("cooldown", cooldown_until as i64))
            .await?;

        Ok(())
    }

    pub async fn record_tokens(&self, account_id: &str, tokens: u64) -> Result<()> {
        self.db
            .query(
                r#"
                UPDATE account SET 
                    tokens_used_today = tokens_used_today + $tokens,
                    consecutive_429_count = 0
                WHERE id = $id
            "#,
            )
            .bind(("id", account_id.to_string()))
            .bind(("tokens", tokens as i64))
            .await?;
        Ok(())
    }

    pub async fn save_session_relay(
        &self,
        session_id: &str,
        summary: &str,
        token_count: usize,
    ) -> Result<()> {
        self.db
            .query(
                r#"
                UPSERT session_relay SET 
                    session_id = $session_id,
                    handoff_summary = $summary,
                    token_count = $token_count,
                    updated_at = time::now()
                WHERE session_id = $session_id
            "#,
            )
            .bind(("session_id", session_id.to_string()))
            .bind(("summary", summary.to_string()))
            .bind(("token_count", token_count as i64))
            .await?;
        Ok(())
    }

    pub async fn get_session_relay(&self, session_id: &str) -> Result<Option<SessionRelayRecord>> {
        let mut res = self
            .db
            .query("SELECT * FROM session_relay WHERE session_id = $session_id")
            .bind(("session_id", session_id.to_string()))
            .await?;
        let mut recs: Vec<SessionRelayRecord> = res.take(0).unwrap_or_default();
        Ok(recs.pop())
    }

    pub async fn list_models(&self) -> Result<Vec<ModelRecord>> {
        let mut res = self.db.query("SELECT * FROM model").await?;
        let models: Vec<ModelRecord> = res.take(0).unwrap_or_default();
        Ok(models)
    }

    async fn seed_default_combos(&self) -> Result<()> {
        let combos = vec![
            ComboRecord {
                id: None,
                name: "auto".to_string(),
                strategy: "lkgp".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-3-7-sonnet".into(),
                        account_id: None,
                        weight: Some(0.4),
                    },
                    ComboStep {
                        provider: "openai".into(),
                        model: "gpt-4o".into(),
                        account_id: None,
                        weight: Some(0.3),
                    },
                    ComboStep {
                        provider: "google".into(),
                        model: "gemini-2.0-flash".into(),
                        account_id: None,
                        weight: Some(0.2),
                    },
                    ComboStep {
                        provider: "deepseek".into(),
                        model: "deepseek-chat".into(),
                        account_id: None,
                        weight: Some(0.1),
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/coding".to_string(),
                strategy: "priority".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-3-7-sonnet".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "qwen".into(),
                        model: "qwen-2.5-coder-32b".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "deepseek".into(),
                        model: "deepseek-reasoner".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "mistral".into(),
                        model: "codestral".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/fast".to_string(),
                strategy: "least-latency".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "cerebras".into(),
                        model: "llama-3.1-8b".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "groq".into(),
                        model: "llama-3.3-70b".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "google".into(),
                        model: "gemini-2.0-flash-lite-001".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/cheap".to_string(),
                strategy: "cost-optimized".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "google".into(),
                        model: "gemini-2.0-flash-thinking-exp:free".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "deepseek".into(),
                        model: "deepseek-chat:free".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "meta-llama".into(),
                        model: "llama-3.3-70b-instruct:free".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "deepseek".into(),
                        model: "deepseek-chat".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/smart".to_string(),
                strategy: "quality-scoring".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-opus-5".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-3-7-sonnet".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "openai".into(),
                        model: "gpt-5.6-sol".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "openai".into(),
                        model: "o1".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "google".into(),
                        model: "gemini-2.5-pro".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/offline".to_string(),
                strategy: "offline-first".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "local".into(),
                        model: "qwen2.5-coder:14b".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "local".into(),
                        model: "deepseek-r1:14b".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 90,
            },
            ComboRecord {
                id: None,
                name: "auto/lkgp".to_string(),
                strategy: "lkgp".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-3-7-sonnet".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "openai".into(),
                        model: "gpt-4o".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
            ComboRecord {
                id: None,
                name: "auto/chaos".to_string(),
                strategy: "chaos".to_string(),
                targets: vec![
                    ComboStep {
                        provider: "anthropic".into(),
                        model: "claude-3-7-sonnet".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "openai".into(),
                        model: "gpt-4o".into(),
                        account_id: None,
                        weight: None,
                    },
                    ComboStep {
                        provider: "google".into(),
                        model: "gemini-2.0-flash".into(),
                        account_id: None,
                        weight: None,
                    },
                ],
                handoff_threshold_percent: 85,
            },
        ];

        for c in combos {
            let name = c.name.clone();
            let _: Option<ComboRecord> = self
                .db
                .create(("combo", name))
                .content(c)
                .await
                .ok()
                .flatten();
        }
        Ok(())
    }

    async fn seed_model_catalog(&self) -> Result<()> {
        let models = vec![
            ModelRecord {
                id: None,
                model_id: "claude-opus-5".into(),
                canonical_name: "Claude Opus 5".into(),
                provider: "anthropic".into(),
                context_window: 500_000,
                supports_thinking: true,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "claude-3-7-sonnet".into(),
                canonical_name: "Claude 3.7 Sonnet (Thinking)".into(),
                provider: "anthropic".into(),
                context_window: 200_000,
                supports_thinking: true,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "gpt-5.6-sol".into(),
                canonical_name: "GPT-5.6 Sol".into(),
                provider: "openai".into(),
                context_window: 1_000_000,
                supports_thinking: true,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "gpt-4o".into(),
                canonical_name: "GPT-4o Omni".into(),
                provider: "openai".into(),
                context_window: 128_000,
                supports_thinking: false,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "deepseek-reasoner".into(),
                canonical_name: "DeepSeek R1".into(),
                provider: "deepseek".into(),
                context_window: 128_000,
                supports_thinking: true,
                supports_vision: false,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "deepseek-chat".into(),
                canonical_name: "DeepSeek V3".into(),
                provider: "deepseek".into(),
                context_window: 128_000,
                supports_thinking: false,
                supports_vision: false,
                supports_tools: true,
                is_free_tier: true,
            },
            ModelRecord {
                id: None,
                model_id: "gemini-2.5-pro".into(),
                canonical_name: "Gemini 2.5 Pro".into(),
                provider: "google".into(),
                context_window: 2_000_000,
                supports_thinking: true,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "gemini-2.0-flash".into(),
                canonical_name: "Gemini 2.0 Flash".into(),
                provider: "google".into(),
                context_window: 1_000_000,
                supports_thinking: false,
                supports_vision: true,
                supports_tools: true,
                is_free_tier: true,
            },
            ModelRecord {
                id: None,
                model_id: "qwen-2.5-coder-32b".into(),
                canonical_name: "Qwen 2.5 Coder 32B".into(),
                provider: "qwen".into(),
                context_window: 128_000,
                supports_thinking: false,
                supports_vision: false,
                supports_tools: true,
                is_free_tier: false,
            },
            ModelRecord {
                id: None,
                model_id: "llama-3.3-70b".into(),
                canonical_name: "Llama 3.3 70B Instruct".into(),
                provider: "meta-llama".into(),
                context_window: 128_000,
                supports_thinking: false,
                supports_vision: false,
                supports_tools: true,
                is_free_tier: true,
            },
            ModelRecord {
                id: None,
                model_id: "codestral".into(),
                canonical_name: "Codestral 2501".into(),
                provider: "mistral".into(),
                context_window: 256_000,
                supports_thinking: false,
                supports_vision: false,
                supports_tools: true,
                is_free_tier: false,
            },
        ];

        for m in models {
            let id = m.model_id.clone();
            let _: Option<ModelRecord> = self
                .db
                .create(("model", id))
                .content(m)
                .await
                .ok()
                .flatten();
        }
        Ok(())
    }
}
