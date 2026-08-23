use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitesurfRenderRequest {
    pub url: String,
    pub wait_for: Option<String>,
    pub reject_resource_types: Option<Vec<String>>,
    pub custom_js: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitesurfContentResponse {
    pub success: bool,
    pub content: Option<String>,
    pub status: Option<u16>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitesurfScreenshotResponse {
    pub success: bool,
    pub image_base64: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct KitesurfClient {
    client: Client,
    account_id: String,
    api_token: String,
    endpoint_template: String,
}

impl KitesurfClient {
    pub fn new(account_id_env: &str, api_token_env: &str, endpoint: &str) -> Self {
        let account_id = std::env::var(account_id_env).unwrap_or_default();
        let api_token = std::env::var(api_token_env).unwrap_or_default();

        let client = Client::builder()
            .timeout(Duration::from_secs(45))
            .build()
            .expect("Failed to build reqwest client for Kitesurf");

        Self {
            client,
            account_id,
            api_token,
            endpoint_template: endpoint.to_string(),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.account_id.is_empty() && !self.api_token.is_empty()
    }

    fn build_url(&self, path: &str) -> String {
        let base = self
            .endpoint_template
            .replace("{account_id}", &self.account_id);
        format!("{}/{}", base.trim_end_matches('/'), path.trim_start_matches('/'))
    }

    /// Extract rendered DOM/Markdown via Cloudflare Browser Run V8 isolate
    pub async fn fetch_rendered_content(&self, url: &str) -> Result<String> {
        if !self.is_configured() {
            return Err(anyhow!(
                "Cloudflare Kitesurf credentials not configured in environment (CLOUDFLARE_ACCOUNT_ID, CLOUDFLARE_API_TOKEN)"
            ));
        }

        let endpoint = self.build_url("content");
        info!("Kitesurf V8 isolate extracting content for: {}", url);

        let body = serde_json::json!({
            "url": url,
            "rejectResourceTypes": ["image", "media", "font"],
        });

        let resp = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Kitesurf API error HTTP {}: {}", status, err_text));
        }

        let html = resp.text().await?;
        Ok(html)
    }

    /// Capture high-fidelity screenshot via Cloudflare Browser Run
    pub async fn capture_screenshot(&self, url: &str, full_page: bool) -> Result<Vec<u8>> {
        if !self.is_configured() {
            return Err(anyhow!(
                "Cloudflare Kitesurf credentials not configured in environment"
            ));
        }

        let endpoint = self.build_url("screenshot");
        info!("Kitesurf capturing screenshot for: {}", url);

        let body = serde_json::json!({
            "url": url,
            "screenshotOptions": {
                "fullPage": full_page,
                "type": "webp"
            }
        });

        let resp = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Kitesurf screenshot error HTTP {}: {}", status, err_text));
        }

        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Convert page/whitepaper to PDF via Cloudflare Browser Run
    pub async fn export_pdf(&self, url: &str) -> Result<Vec<u8>> {
        if !self.is_configured() {
            return Err(anyhow!("Cloudflare Kitesurf credentials not configured"));
        }

        let endpoint = self.build_url("pdf");
        let body = serde_json::json!({ "url": url });

        let resp = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.api_token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow!("Kitesurf PDF export failed: HTTP {}", resp.status()));
        }

        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}
