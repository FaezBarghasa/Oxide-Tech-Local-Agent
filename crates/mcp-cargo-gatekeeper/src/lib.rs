// CargoGatekeeper MCP server

use std::path::{PathBuf, Path};
use std::sync::Arc;
use serde::Deserialize;
use schemars::JsonSchema;
use tracing::{info, warn};
use rmcp::{tool, tool_router, ServerHandler, Error as McpError, model::{CallToolResult, Content, CallToolRequestParam, ListToolsResult, PaginatedRequestParam, ServerInfo}, service::RequestContext, handler::server::tool::{ToolRouter, ToolCallContext}};
use sandbox::execute_in_sandbox;
use tokio::fs;
use regex::Regex;

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
}

#[tool_router]
impl CargoGatekeeperServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    #[tool(description = "Run cargo clippy with -D warnings inside the sandbox")]
    async fn clippy_gate(&self, input: ClippyInput) -> Result<CallToolResult, McpError> {
        let dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = dir.to_string_lossy().to_string();
        match execute_in_sandbox(&["cargo", "clippy", "--", "-D", "warnings"], &dir_str).await {
            Ok(res) => {
                let text = format!("exit code: {}\nstdout:\n{}\nstderr:\n{}", res.exit_code, res.stdout, res.stderr);
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![Content::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![Content::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("Sandbox execution failed: {}", e))])),
        }
    }

    #[tool(description = "Run cargo geiger to report unsafe usage")]
    async fn geiger_report(&self, input: GeigerInput) -> Result<CallToolResult, McpError> {
        let dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = dir.to_string_lossy().to_string();
        // Ensure cargo-geiger is installed; let it fail naturally if missing
        match execute_in_sandbox(&["cargo", "geiger", "--output-format", "Ascii"], &dir_str).await {
            Ok(res) => {
                let text = format!("exit code: {}\nstdout:\n{}\nstderr:\n{}", res.exit_code, res.stdout, res.stderr);
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![Content::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![Content::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("Sandbox execution failed: {}", e))])),
        }
    }

    #[tool(description = "Audit unsafe blocks for // SAFETY: justification. Fails in medical_device_mode.")]
    async fn unsafe_audit(&self, input: UnsafeAuditInput) -> Result<CallToolResult, McpError> {
        let base_dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let mut violations = Vec::new();
        // Recursively walk .rs files
        let mut dirs = vec![base_dir.clone()];
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
                        let re = Regex::new(r"(?m)^\s*unsafe\s+(fn\s|\{)").unwrap();
                        for mat in re.find_iter(&content) {
                            // Determine line number
                            let line_num = content[..mat.start()].matches('\n').count() + 1;
                            // Look back up to 3 lines for SAFETY comment
                            let start_slice = if mat.start() >= 200 { &content[mat.start() - 200..mat.start()] } else { &content[..mat.start()] };
                            let has_comment = start_slice.lines().rev().take(3).any(|l| l.trim_start().starts_with("// SAFETY:"));
                            if !has_comment || input.medical_device_mode {
                                violations.push(format!("{}:{}", path.display(), line_num));
                            }
                        }
                    }
                }
            }
        }
        if violations.is_empty() {
            Ok(CallToolResult::success(vec![Content::text("No unsafe violations found".to_string())]))
        } else {
            let msg = format!("Unsafe violations detected ({}):\n{}", violations.len(), violations.join("\n"));
            Ok(CallToolResult::error(vec![Content::text(msg)]))
        }
    }
}

impl ServerHandler for CargoGatekeeperServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo { name: "Oxide-Tech-mcp-cargo-gatekeeper".to_string(), version: "0.1.0".to_string() }
    }
    async fn list_tools(&self, _request: Option<PaginatedRequestParam>, _context: RequestContext<rmcp::RoleServer>) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult { tools: self.tool_router.list_all(), next_cursor: None })
    }
    async fn call_tool(&self, request: CallToolRequestParam, context: RequestContext<rmcp::RoleServer>) -> Result<CallToolResult, McpError> {
        let call_ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(call_ctx).await
    }
}
