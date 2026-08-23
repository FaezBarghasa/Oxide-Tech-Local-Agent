use reqwest::Client;
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::time::Duration;
use tracing::{info, warn};

pub struct BrowserAgent {
    client: Client,
}

impl BrowserAgent {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) OxideTechAgent/1.0")
            .timeout(Duration::from_secs(15))
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    /// Autonomous web documentation fetching and Markdown content extraction
    pub async fn fetch_page_markdown(&self, url: &str) -> Result<String, String> {
        info!("Autonomous BrowserAgent fetching: {}", url);
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("HTTP request to {} failed: {}", url, e))?;

        let html = resp
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let doc = Document::from(html.as_str());

        // Extract title
        let title = doc
            .find(Name("title"))
            .next()
            .map(|n| n.text())
            .unwrap_or_else(|| "Untitled Page".to_string());

        // Extract main text content from body paragraphs and headers
        let mut markdown_lines = vec![format!("# {}\n\nSource: {}\n", title.trim(), url)];

        for node in doc.find(Name("h1").or(Name("h2")).or(Name("h3")).or(Name("p")).or(Name("pre")).or(Name("code"))) {
            let tag = node.name().unwrap_or("");
            let text = node.text();
            let trimmed = text.trim();
            if trimmed.is_empty() {
                continue;
            }

            match tag {
                "h1" => markdown_lines.push(format!("\n# {}\n", trimmed)),
                "h2" => markdown_lines.push(format!("\n## {}\n", trimmed)),
                "h3" => markdown_lines.push(format!("\n### {}\n", trimmed)),
                "pre" | "code" => markdown_lines.push(format!("\n```\n{}\n```\n", trimmed)),
                _ => markdown_lines.push(trimmed.to_string()),
            }
        }

        if markdown_lines.len() <= 1 {
            warn!("Document parser found minimal content; returning raw text snippet");
            let raw_snippet = html.chars().take(2000).collect::<String>();
            return Ok(format!("# {}\n\n{}", title, raw_snippet));
        }

        Ok(markdown_lines.join("\n\n"))
    }
}

impl Default for BrowserAgent {
    fn default() -> Self {
        Self::new()
    }
}
