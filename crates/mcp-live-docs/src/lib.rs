// Live Docs & Perception Research MCP Server
pub mod kitesurf;
pub mod perception_router;
pub mod pinchtab;
pub mod scraper;

use crate::perception_router::PerceptionRouter;
use crate::pinchtab::ActionRequest;
use crate::scraper::{ApiSurface, DocsRsScraper};
use config_loader::{AppConfig, PerceptionConfig};
use rmcp::{
    handler::server::tool::{ToolCallContext, ToolRouter},
    handler::server::wrapper::Parameters,
    model::{
        CallToolRequestParams, CallToolResult, Content, ListToolsResult, PaginatedRequestParams,
        ServerInfo,
    },
    service::RequestContext,
    tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, JsonSchema)]
pub struct CrateLookupInput {
    /// Name of the crate to fetch from docs.rs
    pub crate_name: String,
    /// Optional specific version (defaults to latest)
    pub version: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchInput {
    /// Target URL or technical query URL (e.g. docs, GitHub issue, errata sheet)
    pub url: String,
    /// Preferred perception engine: "auto" | "scrapling" | "pinchtab" | "kitesurf"
    pub engine: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct BrowserActionInput {
    /// Action type: "navigate" | "click" | "type" | "scroll" | "wait"
    pub action: String,
    /// CSS selector or element reference
    pub selector: Option<String>,
    /// Text to input if action is "type"
    pub text: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct VisualVerifyInput {
    /// Target URL to visually snapshot and render
    pub url: String,
}

#[derive(Clone)]
pub struct LiveDocsServer {
    #[allow(dead_code)]
    workspace_root: PathBuf,
    cache: Arc<Mutex<std::collections::HashMap<(String, String), ApiSurface>>>,
    scraper: DocsRsScraper,
    router: PerceptionRouter,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl LiveDocsServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        let perception_cfg = match AppConfig::load_default() {
            Ok(cfg) => cfg.perception,
            Err(_) => PerceptionConfig::default(),
        };

        Self {
            workspace_root,
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
            scraper: DocsRsScraper::new(),
            router: PerceptionRouter::new(perception_cfg),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Lookup the public API surface of a crate (latest version if omitted)")]
    async fn lookup_crate_api(
        &self,
        Parameters(input): Parameters<CrateLookupInput>,
    ) -> Result<CallToolResult, McpError> {
        let version = if let Some(v) = input.version.clone() {
            v
        } else {
            match self.scraper.fetch_latest_version(&input.crate_name).await {
                Ok(v) => v,
                Err(e) => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Failed to get latest version: {}",
                        e
                    ))]));
                }
            }
        };
        let cache_lock = self.cache.lock().await;
        if let Some(api) = cache_lock.get(&(input.crate_name.clone(), version.clone())) {
            return Ok(CallToolResult::success(vec![Content::text(
                api.markdown.clone(),
            )]));
        }
        drop(cache_lock);
        match self
            .scraper
            .fetch_and_parse(&input.crate_name, &version)
            .await
        {
            Ok(api) => {
                let mut cache_lock = self.cache.lock().await;
                cache_lock.insert((input.crate_name.clone(), version.clone()), api.clone());
                Ok(CallToolResult::success(vec![Content::text(api.markdown)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Docs scrape failed: {}",
                e
            ))])),
        }
    }

    #[tool(
        description = "Deep research and perceive any web URL using the Tri-Engine Perception router (Scrapling / PinchTab / Cloudflare Kitesurf)"
    )]
    async fn perception_deep_research(
        &self,
        Parameters(input): Parameters<ResearchInput>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .router
            .fetch_research(&input.url, input.engine.as_deref())
            .await
        {
            Ok(res) => {
                let formatted = format!(
                    "### Perception Research Result\n- **URL**: {}\n- **Engine**: {}\n- **Confidence**: {:.2}\n\n{}\n",
                    res.url, res.engine_used, res.confidence_score, res.markdown_content
                );
                Ok(CallToolResult::success(vec![Content::text(formatted)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Perception research error: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Execute local interactive browser action via PinchTab daemon")]
    async fn perception_browser_action(
        &self,
        Parameters(input): Parameters<BrowserActionInput>,
    ) -> Result<CallToolResult, McpError> {
        let req = ActionRequest {
            action: input.action,
            selector: input.selector,
            element_id: None,
            text: input.text,
            coordinates: None,
        };

        match self.router.interactive_action(req).await {
            Ok(res) => Ok(CallToolResult::success(vec![Content::text(format!(
                "PinchTab Action: {}",
                res.message
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "PinchTab Action Error: {}",
                e
            ))])),
        }
    }

    #[tool(
        description = "Capture visual snapshot and rendered verification screenshot using Kitesurf or PinchTab"
    )]
    async fn perception_visual_verify(
        &self,
        Parameters(input): Parameters<VisualVerifyInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.router.visual_verify_screenshot(&input.url).await {
            Ok(bytes) => {
                let msg = format!(
                    "Captured visual verification screenshot ({} bytes) for {}",
                    bytes.len(),
                    input.url
                );
                Ok(CallToolResult::success(vec![Content::text(msg)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Visual verify failed: {}",
                e
            ))])),
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
