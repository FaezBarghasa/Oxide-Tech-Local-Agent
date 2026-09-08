// Probe-rs MCP server
pub mod hitl;

use crate::hitl::HitlGate;
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
use tracing::info;

#[derive(Deserialize, JsonSchema)]
pub struct EmptyInput {}

#[derive(Deserialize, JsonSchema)]
pub struct ReadRegisterInput {
    pub address: String,
    pub chip: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ReadMemoryInput {
    pub address: String,
    pub length: u64,
    pub chip: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct FlashBinaryInput {
    pub binary_path: String,
    pub chip: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResetTargetInput {
    pub chip: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct EraseChipInput {
    pub chip: String,
}

#[derive(Clone)]
pub struct ProbeRsServer {
    workspace_root: PathBuf,
    hitl: HitlGate,
    tool_router: ToolRouter<Self>,
}

impl ProbeRsServer {
    async fn require_hitl(&self, operation: &str) -> bool {
        info!("HITL confirmation required for {}", operation);
        self.hitl.wait_for_confirmation().await
    }
}

#[tool_router]
impl ProbeRsServer {
    pub fn new(workspace_root: PathBuf, hitl: HitlGate) -> Self {
        Self {
            workspace_root,
            hitl,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Read a register via probe-rs (read‑only, auto‑exec)")]
    async fn read_register(
        &self,
        Parameters(input): Parameters<ReadRegisterInput>,
    ) -> Result<CallToolResult, McpError> {
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        match execute_in_sandbox(
            &["probe-rs", "read", &input.address, "--chip", &input.chip],
            &dir_str,
        )
        .await
        {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Read a memory region via probe-rs (read‑only, auto‑exec)")]
    async fn read_memory(
        &self,
        Parameters(input): Parameters<ReadMemoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        let length_str = input.length.to_string();
        match execute_in_sandbox(
            &[
                "probe-rs",
                "read",
                &input.address,
                "--length",
                &length_str,
                "--chip",
                &input.chip,
            ],
            &dir_str,
        )
        .await
        {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "List connected probe‑rs devices (read‑only)")]
    async fn list_probes(
        &self,
        _input: Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError> {
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        match execute_in_sandbox(&["probe-rs", "list"], &dir_str).await {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Flash a binary to the target chip (requires human confirmation)")]
    async fn flash_binary(
        &self,
        Parameters(input): Parameters<FlashBinaryInput>,
    ) -> Result<CallToolResult, McpError> {
        if !self.require_hitl("flash_binary").await {
            return Ok(CallToolResult::error(vec![Content::text(
                "HITL confirmation timeout or rejected for flash_binary".to_string(),
            )]));
        }
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        match execute_in_sandbox(
            &[
                "probe-rs",
                "download",
                &input.binary_path,
                "--chip",
                &input.chip,
            ],
            &dir_str,
        )
        .await
        {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Reset the target chip (requires human confirmation)")]
    async fn reset_target(
        &self,
        Parameters(input): Parameters<ResetTargetInput>,
    ) -> Result<CallToolResult, McpError> {
        if !self.require_hitl("reset_target").await {
            return Ok(CallToolResult::error(vec![Content::text(
                "HITL confirmation timeout or rejected for reset_target".to_string(),
            )]));
        }
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        match execute_in_sandbox(&["probe-rs", "reset", "--chip", &input.chip], &dir_str).await {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Erase the target chip flash (requires human confirmation)")]
    async fn erase_chip(
        &self,
        Parameters(input): Parameters<EraseChipInput>,
    ) -> Result<CallToolResult, McpError> {
        if !self.require_hitl("erase_chip").await {
            return Ok(CallToolResult::error(vec![Content::text(
                "HITL confirmation timeout or rejected for erase_chip".to_string(),
            )]));
        }
        let dir_str = self.workspace_root.to_string_lossy().to_string();
        match execute_in_sandbox(&["probe-rs", "erase", "--chip", &input.chip], &dir_str).await {
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
                "Sandbox exec failed: {}",
                e
            ))])),
        }
    }
}

impl ServerHandler for ProbeRsServer {
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
