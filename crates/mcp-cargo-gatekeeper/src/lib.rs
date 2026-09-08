// CargoGatekeeper MCP server

use regex::Regex;
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
use sandbox::execute_in_sandbox;
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;

#[derive(Deserialize, JsonSchema)]
pub struct ClippyInput {
    /// Optional workspace relative path
    pub workspace: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GeigerInput {
    pub workspace: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct UnsafeAuditInput {
    /// If true, any unsafe block causes failure regardless of justification
    pub medical_device_mode: bool,
    /// Optional workspace path (defaults to root)
    pub workspace: Option<String>,
}

#[derive(Clone)]
pub struct CargoGatekeeperServer {
    workspace_root: PathBuf,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CargoGatekeeperServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Run cargo clippy with -D warnings inside the sandbox")]
    async fn clippy_gate(
        &self,
        Parameters(input): Parameters<ClippyInput>,
    ) -> Result<CallToolResult, McpError> {
        let dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = dir.to_string_lossy().to_string();
        match execute_in_sandbox(&["cargo", "clippy", "--", "-D", "warnings"], &dir_str).await {
            Ok(res) => {
                let text = format!(
                    "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                    res.exit_code, res.stdout, res.stderr
                );
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![Content::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![Content::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Sandbox execution failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Run cargo geiger to report unsafe usage")]
    async fn geiger_report(
        &self,
        Parameters(input): Parameters<GeigerInput>,
    ) -> Result<CallToolResult, McpError> {
        let dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = dir.to_string_lossy().to_string();
        match execute_in_sandbox(&["cargo", "geiger", "--output-format", "Ascii"], &dir_str).await {
            Ok(res) => {
                let text = format!(
                    "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                    res.exit_code, res.stdout, res.stderr
                );
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![Content::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![Content::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Sandbox execution failed: {}",
                e
            ))])),
        }
    }

    #[tool(
        description = "Audit unsafe blocks for // SAFETY: justification. Fails in medical_device_mode."
    )]
    async fn unsafe_audit(
        &self,
        Parameters(input): Parameters<UnsafeAuditInput>,
    ) -> Result<CallToolResult, McpError> {
        let base_dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let mut violations = Vec::new();
        let mut dirs = vec![base_dir.clone()];
        let unsafe_re = match Regex::new(r"(?m)^\s*unsafe\s+(fn\s|\{)") {
            Ok(r) => r,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Failed to compile safety audit regex: {}",
                    e
                ))]));
            }
        };

        while let Some(dir) = dirs.pop() {
            let mut entries = match fs::read_dir(&dir).await {
                Ok(e) => e,
                Err(_) => continue,
            };
            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        let content = match fs::read_to_string(&path).await {
                            Ok(c) => c,
                            Err(_) => continue,
                        };
                        for mat in unsafe_re.find_iter(&content) {
                            let line_num = content[..mat.start()].matches('\n').count() + 1;
                            let start_slice = if mat.start() >= 200 {
                                &content[mat.start() - 200..mat.start()]
                            } else {
                                &content[..mat.start()]
                            };
                            let has_comment = start_slice
                                .lines()
                                .rev()
                                .take(3)
                                .any(|l| l.trim_start().starts_with("// SAFETY:"));
                            if !has_comment || input.medical_device_mode {
                                violations.push(format!("{}:{}", path.display(), line_num));
                            }
                        }
                    }
                }
            }
        }
        if violations.is_empty() {
            Ok(CallToolResult::success(vec![Content::text(
                "No unsafe violations found".to_string(),
            )]))
        } else {
            let msg = format!(
                "Unsafe violations detected ({}):\n{}",
                violations.len(),
                violations.join("\n")
            );
            Ok(CallToolResult::error(vec![Content::text(msg)]))
        }
    }
}

impl ServerHandler for CargoGatekeeperServer {
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
        let call_ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(call_ctx).await
    }
}
