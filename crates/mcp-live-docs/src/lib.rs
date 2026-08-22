// Live Docs MCP server
pub mod scraper;

use std::path::PathBuf;
use std::sync::Arc;
use serde::Deserialize;
use schemars::JsonSchema;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::{ToolCallContext, ToolRouter},
    handler::server::wrapper::Parameters,
    model::{
        CallToolRequestParams, CallToolResult, Content, ListToolsResult, PaginatedRequestParams,
        ServerInfo,
    },
    service::RequestContext,
    tool, tool_router,
};
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
    cache: Arc<Mutex<std::collections::HashMap<(String, String), ApiSurface>>>,
    scraper: DocsRsScraper,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl LiveDocsServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
            scraper: DocsRsScraper::new(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Lookup the public API surface of a crate (latest version if omitted)")]
    async fn lookup_crate_api(&self, Parameters(input): Parameters<CrateLookupInput>) -> Result<CallToolResult, McpError> {
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
        let mut cache_lock = self.cache.lock().await;
        if let Some(api) = cache_lock.get(&(input.crate_name.clone(), version.clone())) {
            return Ok(CallToolResult::success(vec![Content::text(api.markdown.clone())]));
        }
        drop(cache_lock);
        match self.scraper.fetch_and_parse(&input.crate_name, &version).await {
            Ok(api) => {
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
        ServerInfo::default()
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult {
            tools: self.tool_router.list_all(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}
