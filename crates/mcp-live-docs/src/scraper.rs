// Live Docs scraper implementation

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use anyhow::Result;
use reqwest::Client;
use select::document::Document;
use select::predicate::Class;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct ApiSurface {
    pub markdown: String,
}

#[derive(Clone)]
pub struct DocsRsScraper {
    client: Client,
    // Simple rate‑limit: ensure at least `rate_limit_delay_ms` between requests
    last_request: Arc<Mutex<Instant>>,
    rate_delay: Duration,
}

impl Default for DocsRsScraper {
    fn default() -> Self {
        Self::new()
    }
}

impl DocsRsScraper {
    pub fn new() -> Self {
        // Use rustls and a short timeout
        let client = Client::builder()
            .user_agent("oxide-tech-live-docs/0.1.0")
            .timeout(Duration::from_secs(15))
            .build()
            .expect("Failed to build reqwest client");
        Self {
            client,
            last_request: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1))),
            rate_delay: Duration::from_millis(1000), // default 1 s per request
        }
    }

    async fn rate_limit(&self) {
        let mut guard = self.last_request.lock().await;
        let elapsed = guard.elapsed();
        if elapsed < self.rate_delay {
            tokio::time::sleep(self.rate_delay - elapsed).await;
        }
        *guard = Instant::now();
    }

    /// Resolve the latest version from crates.io. Returns a string like "1.2.3".
    pub async fn fetch_latest_version(&self, crate_name: &str) -> Result<String> {
        self.rate_limit().await;
        let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        #[derive(Deserialize)]
        struct CrateInfo {
            max_version: String,
        }
        #[derive(Deserialize)]
        struct CrateResponse {
            #[serde(rename = "crate")]
            krate: CrateInfo,
        }
        let data: CrateResponse = resp.json().await?;
        Ok(data.krate.max_version)
    }

    /// Fetch the docs.rs index page for the given crate+version and parse an API surface.
    pub async fn fetch_and_parse(&self, crate_name: &str, version: &str) -> Result<ApiSurface> {
        self.rate_limit().await;
        // Docs.rs URL pattern – the index page lists modules and items.
        let url = format!("https://docs.rs/{}/{}/index.html", crate_name, version);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let html = resp.text().await?;
        let api = self.parse_api_surface(&html, crate_name, version);
        Ok(api)
    }

    fn parse_api_surface(&self, html: &str, crate_name: &str, version: &str) -> ApiSurface {
        let document = Document::from(html);
        // Grab all public items: structs, enums, traits, functions, type aliases.
        // Docs.rs uses the CSS class "item-decl" for the code snippet, and
        // "docblock" for the first paragraph of documentation.
        let mut sections = Vec::new();
        for node in document.find(Class("item-decl")) {
            // Extract the raw Rust signature text – it appears as plain text within the element.
            let signature = node.text();
            // Find the surrounding docblock (if any) – look for the next sibling with class "docblock".
            let doc = node
                .next()
                .filter(|n| n.is(Class("docblock")))
                .map(|n| n.text())
                .unwrap_or_default();
            let mut md = format!("```rust\n{}\n```\n", signature.trim());
            if !doc.is_empty() {
                md.push_str(&format!("_{}_\n", doc.trim()));
            }
            sections.push(md);
        }
        // If nothing was found, fallback to a simple notice.
        let body = if sections.is_empty() {
            "*No public API items were detected on the docs.rs index page.*".to_string()
        } else {
            sections.join("\n---\n")
        };
        let markdown = format!("# {} v{} API surface\n\n{}", crate_name, version, body);
        ApiSurface { markdown }
    }
}
