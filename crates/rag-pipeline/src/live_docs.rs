use std::sync::Arc;
use crate::RagPipeline;
use anyhow::Result;
use tracing::info;

/// Helper for scraping docs.rs for a crate and ingesting it into the RAG pipeline.
/// This is primarily used by the MCP `live_docs_scrape` tool.
pub struct LiveDocsScraper {
    pub pipeline: Arc<RagPipeline>,
}

impl LiveDocsScraper {
    pub fn new(pipeline: Arc<RagPipeline>) -> Self {
        Self { pipeline }
    }

    /// Scrape the latest docs for the given crate name and ingest.
    /// For simplicity, we delegate to `RagPipeline::ingest_crate_docs` with version "latest".
    pub async fn scrape_and_ingest(&self, crate_name: &str, version: &str) -> Result<()> {
        info!("LiveDocsScraper: scraping crate {} from docs.rs", crate_name);
        // Use "latest" version; the RagPipeline will resolve the latest version internally.
        self.pipeline.ingest_crate_docs(crate_name, "latest").await
    }
}
