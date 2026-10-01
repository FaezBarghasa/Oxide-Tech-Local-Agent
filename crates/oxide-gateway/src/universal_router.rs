use crate::db::{AccountRecord, ComboRecord, GatewayDb};
use anyhow::{anyhow, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutedTarget {
    pub provider: String,
    pub model: String,
    pub is_browser_session: bool,
    pub supports_thinking: bool,
    pub context_window: u32,
    pub account: Option<AccountRecord>,
    pub routing_strategy_applied: String,
}

pub struct ModelResolver;

impl ModelResolver {
    /// Normalizes incoming model IDs, aliases, and virtual combos into a concrete target
    pub async fn resolve(
        requested_model: &str,
        db: &Arc<GatewayDb>,
        round_robin: &Arc<AtomicUsize>,
    ) -> Result<RoutedTarget> {
        let trimmed = requested_model.trim();

        // 1. Direct combo or openrouter/auto check
        if trimmed.starts_with("auto") || trimmed.starts_with("openrouter/auto") || trimmed.starts_with("openrouter/flavor-of-the-week") {
            let combo_name = match trimmed {
                "openrouter/auto" | "openrouter/flavor-of-the-week" => "auto",
                other => other,
            };

            if let Some(combo) = db.get_combo(combo_name).await? {
                return Self::evaluate_combo(&combo, db, round_robin).await;
            }
        }

        // 2. Parse Provider-Prefixed IDs (e.g. "anthropic/claude-3.7-sonnet:thinking", "deepseek/deepseek-r1:free")
        if let Some((provider, model_name)) = trimmed.split_once('/') {
            let clean_model = model_name.split(':').next().unwrap_or(model_name);
            let is_browser = Self::check_if_browser_required(provider, clean_model);
            let account = db.get_healthy_account(provider).await?;
            let (supports_thinking, ctx) = Self::infer_capabilities(clean_model);

            return Ok(RoutedTarget {
                provider: provider.to_string(),
                model: clean_model.to_string(),
                is_browser_session: is_browser,
                supports_thinking,
                context_window: ctx,
                account,
                routing_strategy_applied: "direct-prefix".into(),
            });
        }

        // 3. Normalized Core IDs (e.g., "claude-3-7-sonnet", "gpt-4o", "deepseek-reasoner", "gemini-2.5-pro")
        let (provider, upstream_model, supports_thinking, ctx) = Self::map_core_id_to_provider(trimmed);
        let is_browser = Self::check_if_browser_required(&provider, &upstream_model);
        let account = db.get_healthy_account(&provider).await?;

        Ok(RoutedTarget {
            provider,
            model: upstream_model,
            is_browser_session: is_browser,
            supports_thinking,
            context_window: ctx,
            account,
            routing_strategy_applied: "canonical-alias".into(),
        })
    }

    pub fn map_core_id_to_provider(core_id: &str) -> (String, String, bool, u32) {
        match core_id {
            // Anthropic Frontier
            "claude-opus-5" => ("anthropic".into(), "claude-opus-5".into(), true, 500_000),
            "claude-opus-4-8" => ("anthropic".into(), "claude-opus-4-8".into(), true, 200_000),
            "claude-3-7-sonnet" | "claude-3.7-sonnet" => ("anthropic".into(), "claude-3-7-sonnet-20250219".into(), true, 200_000),
            "claude-3-5-sonnet" | "claude-3.5-sonnet" => ("anthropic".into(), "claude-3-5-sonnet-20241022".into(), false, 200_000),
            "claude-3-5-haiku" | "claude-3.5-haiku" => ("anthropic".into(), "claude-3-5-haiku-20241022".into(), false, 200_000),

            // OpenAI Frontier & Reasoning
            "gpt-5.6-sol" => ("openai".into(), "gpt-5.6-sol".into(), true, 1_000_000),
            "gpt-4o" => ("openai".into(), "gpt-4o".into(), false, 128_000),
            "gpt-4o-mini" => ("openai".into(), "gpt-4o-mini".into(), false, 128_000),
            "o3-mini" => ("openai".into(), "o3-mini".into(), true, 200_000),
            "o1" => ("openai".into(), "o1".into(), true, 200_000),
            "o1-mini" => ("openai".into(), "o1-mini".into(), true, 128_000),

            // DeepSeek
            "deepseek-chat" | "deepseek-v3" => ("deepseek".into(), "deepseek-chat".into(), false, 128_000),
            "deepseek-reasoner" | "deepseek-r1" => ("deepseek".into(), "deepseek-reasoner".into(), true, 128_000),
            "deepseek-coder" => ("deepseek".into(), "deepseek-coder".into(), false, 128_000),

            // Google DeepMind
            "gemini-2.5-pro" | "gemini-2.5-pro-exp" => ("google".into(), "gemini-2.5-pro".into(), true, 2_000_000),
            "gemini-2.0-flash" | "gemini-2.0-flash-001" => ("google".into(), "gemini-2.0-flash".into(), false, 1_000_000),
            "gemini-2.0-flash-lite" => ("google".into(), "gemini-2.0-flash-lite-001".into(), false, 1_000_000),
            "gemini-1.5-pro" => ("google".into(), "gemini-1.5-pro".into(), false, 2_000_000),
            "gemini-1.5-flash" => ("google".into(), "gemini-1.5-flash".into(), false, 1_000_000),

            // Meta Open Foundation
            "llama-3.3-70b" => ("meta-llama".into(), "llama-3.3-70b-instruct".into(), false, 128_000),
            "llama-3.1-405b" => ("meta-llama".into(), "llama-3.1-405b-instruct".into(), false, 128_000),
            "llama-3.1-70b" => ("meta-llama".into(), "llama-3.1-70b-instruct".into(), false, 128_000),
            "llama-3.1-8b" => ("meta-llama".into(), "llama-3.1-8b-instruct".into(), false, 128_000),

            // Alibaba Qwen
            "qwen-2.5-coder-32b" | "qwen-2.5-coder" => ("qwen".into(), "qwen-2.5-coder-32b-instruct".into(), false, 128_000),
            "qwen-2.5-72b" => ("qwen".into(), "qwen-2.5-72b-instruct".into(), false, 128_000),
            "qwen-turbo" => ("qwen".into(), "qwen-turbo".into(), false, 128_000),
            "qwen-plus" => ("qwen".into(), "qwen-plus".into(), false, 128_000),
            "qwen-max" => ("qwen".into(), "qwen-max".into(), true, 128_000),
            "qwq-32b" => ("qwen".into(), "qwq-32b-preview".into(), true, 128_000),

            // Mistral AI
            "mistral-large" => ("mistralai".into(), "mistral-large-2411".into(), false, 128_000),
            "mistral-small" => ("mistralai".into(), "mistral-small-24b-instruct-2501".into(), false, 32_000),
            "codestral" => ("mistralai".into(), "codestral-2501".into(), false, 256_000),
            "pixtral" => ("mistralai".into(), "pixtral-large-2411".into(), false, 128_000),

            // Zhipu AI
            "glm-4.7" => ("zhipu".into(), "glm-4.7".into(), false, 128_000),
            "glm-4" => ("zhipu".into(), "glm-4".into(), false, 128_000),

            // Moonshot & MiniMax
            "kimi-k3" => ("moonshot".into(), "kimi-k3".into(), false, 2_000_000),
            "kimi-k2" => ("moonshot".into(), "kimi-k2".into(), false, 200_000),
            "minimax-m3" | "minimax-01" => ("minimax".into(), "minimax-m3".into(), false, 1_000_000),

            // Local fallback
            unknown => ("local".into(), unknown.to_string(), false, 32_000),
        }
    }

    fn infer_capabilities(model: &str) -> (bool, u32) {
        let m = model.to_lowercase();
        let supports_thinking = m.contains("reason") || m.contains("r1") || m.contains("o1") || m.contains("o3") || m.contains("thinking") || m.contains("qwq");
        let ctx = if m.contains("2.5-pro") || m.contains("1.5-pro") || m.contains("kimi") {
            2_000_000
        } else if m.contains("sol") || m.contains("flash") || m.contains("minimax") {
            1_000_000
        } else if m.contains("opus") {
            500_000
        } else {
            128_000
        };
        (supports_thinking, ctx)
    }

    fn check_if_browser_required(provider: &str, model: &str) -> bool {
        let p = provider.to_lowercase();
        let m = model.to_lowercase();
        p == "browser" || m.contains("web-session") || m.contains("chatgpt-plus-web")
    }

    async fn evaluate_combo(
        combo: &ComboRecord,
        db: &Arc<GatewayDb>,
        round_robin: &Arc<AtomicUsize>,
    ) -> Result<RoutedTarget> {
        let targets = &combo.targets;
        if targets.is_empty() {
            return Err(anyhow!("Combo '{}' has no configured targets", combo.name));
        }

        match combo.strategy.as_str() {
            "priority" | "quality-scoring" | "cost-optimized" | "least-latency" | "offline-first" => {
                for step in targets {
                    if let Ok(Some(account)) = db.get_healthy_account(&step.provider).await {
                        let (supports_thinking, ctx) = Self::infer_capabilities(&step.model);
                        return Ok(RoutedTarget {
                            provider: step.provider.clone(),
                            model: step.model.clone(),
                            is_browser_session: false,
                            supports_thinking,
                            context_window: ctx,
                            account: Some(account),
                            routing_strategy_applied: combo.strategy.clone(),
                        });
                    }
                }
                // Fallback to first step
                let step = &targets[0];
                let (supports_thinking, ctx) = Self::infer_capabilities(&step.model);
                Ok(RoutedTarget {
                    provider: step.provider.clone(),
                    model: step.model.clone(),
                    is_browser_session: false,
                    supports_thinking,
                    context_window: ctx,
                    account: db.get_healthy_account(&step.provider).await.ok().flatten(),
                    routing_strategy_applied: combo.strategy.clone(),
                })
            }
            "round-robin" => {
                let idx = round_robin.fetch_add(1, Ordering::Relaxed) % targets.len();
                let step = &targets[idx];
                let (supports_thinking, ctx) = Self::infer_capabilities(&step.model);
                Ok(RoutedTarget {
                    provider: step.provider.clone(),
                    model: step.model.clone(),
                    is_browser_session: false,
                    supports_thinking,
                    context_window: ctx,
                    account: db.get_healthy_account(&step.provider).await.ok().flatten(),
                    routing_strategy_applied: "round-robin".into(),
                })
            }
            "chaos" => {
                // Return primary target while signaling fan-out intent
                let step = &targets[0];
                let (supports_thinking, ctx) = Self::infer_capabilities(&step.model);
                Ok(RoutedTarget {
                    provider: step.provider.clone(),
                    model: step.model.clone(),
                    is_browser_session: false,
                    supports_thinking,
                    context_window: ctx,
                    account: db.get_healthy_account(&step.provider).await.ok().flatten(),
                    routing_strategy_applied: "chaos-fanout".into(),
                })
            }
            _ => {
                // LKGP default strategy
                let step = &targets[0];
                let (supports_thinking, ctx) = Self::infer_capabilities(&step.model);
                Ok(RoutedTarget {
                    provider: step.provider.clone(),
                    model: step.model.clone(),
                    is_browser_session: false,
                    supports_thinking,
                    context_window: ctx,
                    account: db.get_healthy_account(&step.provider).await.ok().flatten(),
                    routing_strategy_applied: "lkgp-sticky".into(),
                })
            }
        }
    }
}
