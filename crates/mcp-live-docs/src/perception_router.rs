use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::kitesurf::KitesurfClient;
use crate::pinchtab::{ActionRequest, ActionResponse, PinchTabClient};
use config_loader::PerceptionConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResearchResult {
    pub url: String,
    pub engine_used: String,
    pub success: bool,
    pub title: String,
    pub markdown_content: String,
    pub links: Vec<String>,
    pub confidence_score: f32,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct PerceptionRouter {
    config: PerceptionConfig,
    pinchtab: PinchTabClient,
    kitesurf: KitesurfClient,
    cache: Arc<Mutex<std::collections::HashMap<String, ResearchResult>>>,
}

impl PerceptionRouter {
    pub fn new(config: PerceptionConfig) -> Self {
        let pinchtab = PinchTabClient::new(&config.pinchtab.endpoint, config.pinchtab.cloak_mode);
        let kitesurf = KitesurfClient::new(
            &config.kitesurf.account_id_env,
            &config.kitesurf.api_token_env,
            &config.kitesurf.endpoint,
        );

        Self {
            config,
            pinchtab,
            kitesurf,
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Primary intelligent perception dispatch: Tier 1 (Scrapling/DocsRs) -> Tier 2 (PinchTab) -> Tier 3 (Kitesurf)
    pub async fn fetch_research(
        &self,
        url: &str,
        force_engine: Option<&str>,
    ) -> Result<ResearchResult> {
        let cache_lock = self.cache.lock().await;
        if let Some(cached) = cache_lock.get(url) {
            return Ok(cached.clone());
        }
        drop(cache_lock);

        let engine = force_engine.unwrap_or(&self.config.default_engine);
        info!("Perception router dispatching {} via mode: {}", url, engine);

        let result = match engine {
            "pinchtab" => self.execute_pinchtab_fetch(url).await,
            "kitesurf" => self.execute_kitesurf_fetch(url).await,
            "scrapling" => self.execute_scrapling_fetch(url).await,
            _ => self.execute_auto_hierarchy(url).await,
        }?;

        let mut cache_lock = self.cache.lock().await;
        cache_lock.insert(url.to_string(), result.clone());
        Ok(result)
    }

    async fn execute_auto_hierarchy(&self, url: &str) -> Result<ResearchResult> {
        // Step 1: Try Tier 1 (Scrapling / Local Fast Scraper)
        match self.execute_scrapling_fetch(url).await {
            Ok(res) if res.success => return Ok(res),
            Ok(blocked_res) => {
                warn!(
                    "Tier 1 Scrapling flagged/blocked for {}: {:?}",
                    url, blocked_res.error
                );
                if !self.config.auto_escalate_on_block {
                    return Ok(blocked_res);
                }
            }
            Err(e) => {
                warn!("Tier 1 Scrapling error for {}: {}", url, e);
                if !self.config.auto_escalate_on_block {
                    return Err(e);
                }
            }
        }

        // Step 2: Try Tier 2 (PinchTab Local Daemon) if available
        if self.pinchtab.health_check().await {
            info!(
                "Escalating perception to Tier 2 (PinchTab Daemon) for {}",
                url
            );
            if let Ok(res) = self.execute_pinchtab_fetch(url).await {
                if res.success {
                    return Ok(res);
                }
            }
        }

        // Step 3: Try Tier 3 (Cloudflare Kitesurf Cloud Isolates)
        if self.kitesurf.is_configured() {
            info!(
                "Escalating perception to Tier 3 (Cloudflare Kitesurf V8) for {}",
                url
            );
            return self.execute_kitesurf_fetch(url).await;
        }

        // Return fallback notice
        Ok(ResearchResult {
            url: url.to_string(),
            engine_used: "fallback".to_string(),
            success: false,
            title: "".to_string(),
            markdown_content: "All perception tiers failed or lack credentials".to_string(),
            links: vec![],
            confidence_score: 0.0,
            error: Some("Failed across Scrapling, PinchTab, and Kitesurf".to_string()),
        })
    }

    async fn execute_scrapling_fetch(&self, url: &str) -> Result<ResearchResult> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                self.config.scrapling.request_timeout_secs,
            ))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
            .build()?;

        let resp = client.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Ok(ResearchResult {
                url: url.to_string(),
                engine_used: "scrapling_local".to_string(),
                success: false,
                title: "".to_string(),
                markdown_content: "".to_string(),
                links: vec![],
                confidence_score: 0.0,
                error: Some(format!("HTTP {}", status)),
            });
        }

        let html = resp.text().await?;
        let markdown = format!("# Scraped Content from {}\n\n{}", url, html);

        Ok(ResearchResult {
            url: url.to_string(),
            engine_used: "scrapling_local".to_string(),
            success: true,
            title: "Docs / Page Surface".to_string(),
            markdown_content: markdown,
            links: vec![],
            confidence_score: 0.95,
            error: None,
        })
    }

    async fn execute_pinchtab_fetch(&self, url: &str) -> Result<ResearchResult> {
        let _tab = self.pinchtab.navigate(url).await?;
        let snapshot = self.pinchtab.snapshot(None).await?;

        Ok(ResearchResult {
            url: url.to_string(),
            engine_used: "pinchtab_daemon".to_string(),
            success: snapshot.success,
            title: snapshot.title,
            markdown_content: snapshot.text_content,
            links: vec![],
            confidence_score: 0.98,
            error: snapshot.error,
        })
    }

    async fn execute_kitesurf_fetch(&self, url: &str) -> Result<ResearchResult> {
        let content = self.kitesurf.fetch_rendered_content(url).await?;

        Ok(ResearchResult {
            url: url.to_string(),
            engine_used: "cloudflare_kitesurf".to_string(),
            success: true,
            title: "Kitesurf Rendered DOM".to_string(),
            markdown_content: content,
            links: vec![],
            confidence_score: 0.99,
            error: None,
        })
    }

    pub async fn interactive_action(&self, action: ActionRequest) -> Result<ActionResponse> {
        self.pinchtab.execute_action(action).await
    }

    pub async fn visual_verify_screenshot(&self, url: &str) -> Result<Vec<u8>> {
        if self.kitesurf.is_configured() {
            self.kitesurf.capture_screenshot(url, false).await
        } else if self.pinchtab.health_check().await {
            let _ = self.pinchtab.navigate(url).await?;
            self.pinchtab.capture_screenshot(None).await
        } else {
            Err(anyhow::anyhow!(
                "Neither Kitesurf nor PinchTab available for visual screenshot verification"
            ))
        }
    }
}
