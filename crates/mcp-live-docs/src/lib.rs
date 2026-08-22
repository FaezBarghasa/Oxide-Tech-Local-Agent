// Live Docs MCP server

use std::path::{PathBuf, Path};
use std::sync::Arc;
use serde::Deserialize;
use schemars::JsonSchema;
use tracing::{info, warn};
use rmcp::{tool, tool_router, ServerHandler, Error as McpError, model::{CallToolResult, Content, CallToolRequestParam, ListToolsResult, PaginatedRequestParam, ServerInfo}, service::RequestContext, handler::server::tool::{ToolRouter, ToolCallContext}};
use tokio::sync::Mutex;
use crate::scraper::{DocsRsScraper, ApiSurface};

#[derive(Deserialize, JsonSchema)]
pub struct CrateLookupInput {
    /// Name of the crate to fetch from docs.rs
    pub crate_name: String,
    /// Optional specific version (defaults to latest)
    pub version: Option<String>,
}

#[derive(Clone)]
pub struct LiveDocsServer {
    workspace_root: PathBuf,
    // Simple in‑memory cache: key = (crate, version)
    cache: Arc<Mutex<std::collections::HashMap<(String, String), ApiSurface>>>,
    scraper: DocsRsScraper,
}

#[tool_router]
impl LiveDocsServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
            scraper: DocsRsScraper::new(),
        }
    }

    #[tool(description = "Lookup the public API surface of a crate (latest version if omitted)")]
    async fn lookup_crate_api(&self, input: CrateLookupInput) -> Result<CallToolResult, McpError> {
        // Resolve version via crates.io if not given
        let version = if let Some(v) = input.version.clone() {
            v
        } else {
            match self.scraper.fetch_latest_version(&input.crate_name).await {
                Ok(v) => v,
                Err(e) => {
                    return Ok(CallToolResult::error(vec![Content::text(format!("Failed to get latest version: {}", e))]));
                }
            }
        };
        // Check cache
        let mut cache_lock = self.cache.lock().await;
        if let Some(api) = cache_lock.get(&(input.crate_name.clone(), version.clone())) {
            return Ok(CallToolResult::success(vec![Content::text(api.markdown.clone())]));
        }
        drop(cache_lock); // release lock before async fetch
        // Fetch and parse docs.rs page
        match self.scraper.fetch_and_parse(&input.crate_name, &version).await {
            Ok(api) => {
                // Insert into cache
                let mut cache_lock = self.cache.lock().await;
                cache_lock.insert((input.crate_name.clone(), version.clone()), api.clone());
                Ok(CallToolResult::success(vec![Content::text(api.markdown)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("Docs scrape failed: {}", e))])),
        }
    }
}

impl ServerHandler for LiveDocsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo { name: "Oxide-Tech-mcp-live-docs".to_string(), version: "0.1.0".to_string() }
    }
    async fn list_tools(&self, _request: Option<PaginatedRequestParam>, _context: RequestContext<rmcp::RoleServer>) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult { tools: self.tool_router.list_all(), next_cursor: None })
    }
    async fn call_tool(&self, request: CallToolRequestParam, context: RequestContext<rmcp::RoleServer>) -> Result<CallToolResult, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}
