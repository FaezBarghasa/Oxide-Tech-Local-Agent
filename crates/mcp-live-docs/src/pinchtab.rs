use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinchTabTab {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessibilityNode {
    pub role: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub id: Option<String>,
    pub children: Option<Vec<AccessibilityNode>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotResponse {
    pub success: bool,
    pub url: String,
    pub title: String,
    pub text_content: String,
    pub ax_tree: Option<AccessibilityNode>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionRequest {
    pub action: String, // "click" | "type" | "navigate" | "scroll" | "wait"
    pub selector: Option<String>,
    pub element_id: Option<String>,
    pub text: Option<String>,
    pub coordinates: Option<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct PinchTabClient {
    client: Client,
    endpoint: String,
    cloak_mode: bool,
}

impl PinchTabClient {
    pub fn new(endpoint: &str, cloak_mode: bool) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client for PinchTab");

        Self {
            client,
            endpoint: endpoint.trim_end_matches('/').to_string(),
            cloak_mode,
        }
    }

    /// Check if PinchTab daemon is reachable
    pub async fn health_check(&self) -> bool {
        match self.client.get(format!("{}/health", self.endpoint)).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// List active browser tabs
    pub async fn list_tabs(&self) -> Result<Vec<PinchTabTab>> {
        let resp = self
            .client
            .get(format!("{}/tabs", self.endpoint))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow!("PinchTab /tabs failed: HTTP {}", resp.status()));
        }

        let tabs = resp.json::<Vec<PinchTabTab>>().await?;
        Ok(tabs)
    }

    /// Navigate active tab or create new tab to URL
    pub async fn navigate(&self, url: &str) -> Result<PinchTabTab> {
        info!("PinchTab navigating to URL: {}", url);
        let body = serde_json::json!({
            "url": url,
            "cloak": self.cloak_mode,
        });

        let resp = self
            .client
            .post(format!("{}/navigate", self.endpoint))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("PinchTab navigation failed: {}", err_text));
        }

        let tab = resp.json::<PinchTabTab>().await?;
        Ok(tab)
    }

    /// Get accessibility-tree based page snapshot
    pub async fn snapshot(&self, tab_id: Option<&str>) -> Result<SnapshotResponse> {
        let mut url = format!("{}/snapshot", self.endpoint);
        if let Some(id) = tab_id {
            url = format!("{}?tab_id={}", url, id);
        }

        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("PinchTab /snapshot failed: HTTP {}", resp.status()));
        }

        let snapshot = resp.json::<SnapshotResponse>().await?;
        Ok(snapshot)
    }

    /// Execute interactive browser action
    pub async fn execute_action(&self, action: ActionRequest) -> Result<ActionResponse> {
        let resp = self
            .client
            .post(format!("{}/action", self.endpoint))
            .json(&action)
            .send()
            .await?;

        let res = resp.json::<ActionResponse>().await?;
        Ok(res)
    }

    /// Capture screenshot of rendered page
    pub async fn capture_screenshot(&self, tab_id: Option<&str>) -> Result<Vec<u8>> {
        let mut url = format!("{}/screenshot", self.endpoint);
        if let Some(id) = tab_id {
            url = format!("{}?tab_id={}", url, id);
        }

        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("PinchTab screenshot failed: HTTP {}", resp.status()));
        }

        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}
