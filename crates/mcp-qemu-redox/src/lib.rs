// QEMU Redox MCP server

use std::process::Stdio;
use std::sync::Arc;
use std::path::{PathBuf, Path};
use std::time::Duration;
use tokio::process::{Command, Child};
use tokio::sync::Mutex;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use regex::Regex;
use serde::Deserialize;
use schemars::JsonSchema;
use tracing::{info, warn};
use rmcp::{tool, tool_router, ServerHandler, Error as McpError, model::{CallToolResult, Content, CallToolRequestParam, ListToolsResult, PaginatedRequestParam, ServerInfo}, service::RequestContext, handler::server::tool::{ToolRouter, ToolCallContext}};

#[derive(Deserialize, JsonSchema)]
pub struct BootInput {
    pub image_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct UartInput {
    pub message: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct PanicTestInput {
    pub image_path: String,
    pub trigger_cmd: String,
}

#[derive(Clone)]
pub struct QemuRedoxServer {
    workspace_root: PathBuf,
    // Shared state for the running QEMU instance
    state: Arc<Mutex<QemuState>>, 
}

struct QemuState {
    child: Option<Child>,
    // Accumulated serial output
    log: String,
}

impl QemuRedoxServer {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            state: Arc::new(Mutex::new(QemuState { child: None, log: String::new() })),
        }
    }

    async fn spawn_qemu(&self, image_path: &str) -> Result<(), McpError> {
        let mut state = self.state.lock().await;
        if state.child.is_some() {
            return Ok(()); // Already running
        }
        let full_image = self.workspace_root.join(image_path);
        let image_str = full_image.to_string_lossy();
        // Basic headless QEMU command (x86_64)
        let mut cmd = Command::new("qemu-system-x86_64");
        cmd.arg("-machine").arg("q35")
            .arg("-m").arg("512")
            .arg("-drive").arg(format!("file={},format=raw", image_str))
            .arg("-nographic")
            .arg("-serial").arg("stdio")
            .arg("-monitor").arg("none")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd.spawn().map_err(|e| McpError::internal_error(format!("Failed to spawn QEMU: {}", e), None))?;
        // Capture stdout in background task
        let stdout = child.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout).lines();
        let log_arc = self.state.clone();
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                let mut st = log_arc.lock().await;
                st.log.push_str(&line);
                st.log.push('\n');
            }
        });
        state.child = Some(child);
        Ok(())
    }
}

#[tool_router]
impl QemuRedoxServer {
    #[tool(description = "Boot a Redox OS image in QEMU (headless). Returns success if QEMU started.")]
    async fn boot_redox(&self, input: BootInput) -> Result<CallToolResult, McpError> {
        self.spawn_qemu(&input.image_path).await?;
        Ok(CallToolResult::success(vec![Content::text("QEMU booted successfully".to_string())]))
    }

    #[tool(description = "Send a line to QEMU UART (stdin).")]
    async fn send_uart(&self, input: UartInput) -> Result<CallToolResult, McpError> {
        let mut state = self.state.lock().await;
        if let Some(child) = &mut state.child {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.message.as_bytes()).await.map_err(|e| McpError::internal_error(format!("Failed writing to QEMU stdin: {}", e), None))?;
                stdin.write_all(b"\n").await.map_err(|e| McpError::internal_error(format!("Failed writing newline: {}", e), None))?;
                // Put stdin back
                child.stdin = Some(stdin);
                return Ok(CallToolResult::success(vec![Content::text("UART message sent".to_string())]));
            }
        }
        Ok(CallToolResult::error(vec![Content::text("QEMU not running".to_string())]))
    }

    #[tool(description = "Read accumulated QEMU serial log.")]
    async fn read_serial_log(&self) -> Result<CallToolResult, McpError> {
        let state = self.state.lock().await;
        let log = state.log.clone();
        Ok(CallToolResult::success(vec![Content::text(log)]))
    }

    #[tool(description = "Shutdown the running QEMU instance.")]
    async fn shutdown_qemu(&self) -> Result<CallToolResult, McpError> {
        let mut state = self.state.lock().await;
        if let Some(mut child) = state.child.take() {
            // Try graceful termination
            let _ = child.kill().await;
            let _ = child.wait().await;
            state.log.clear();
            return Ok(CallToolResult::success(vec![Content::text("QEMU shutdown".to_string())]));
        }
        Ok(CallToolResult::error(vec![Content::text("No QEMU instance to shut down".to_string())]))
    }

    #[tool(description = "Run a kernel panic test: boot image, send trigger command, capture and parse stack trace.")]
    async fn run_panic_test(&self, input: PanicTestInput) -> Result<CallToolResult, McpError> {
        // Ensure QEMU is running
        self.spawn_qemu(&input.image_path).await?;
        // Send trigger command
        self.send_uart(UartInput { message: input.trigger_cmd }).await?;
        // Wait a short period for panic output (simple sleep)
        tokio::time::sleep(Duration::from_secs(5)).await;
        // Read log
        let log = {
            let state = self.state.lock().await;
            state.log.clone()
        };
        // Very naive panic parser: look for lines starting with "KERNEL PANIC"
        let re = Regex::new(r"(?m)^KERNEL PANIC: (.*)$").unwrap();
        let mut panic_msg = String::new();
        for cap in re.captures_iter(&log) {
            panic_msg.push_str(&cap[1]);
            panic_msg.push('\n');
        }
        if panic_msg.is_empty() {
            return Ok(CallToolResult::error(vec![Content::text("No panic detected in QEMU output".to_string())]));
        }
        Ok(CallToolResult::success(vec![Content::text(format!("Panic detected:\n{}", panic_msg))]))
    }
}

impl ServerHandler for QemuRedoxServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo { name: "Oxide-Tech-mcp-qemu-redox".to_string(), version: "0.1.0".to_string() }
    }
    async fn list_tools(&self, _request: Option<PaginatedRequestParam>, _context: RequestContext<rmcp::RoleServer>) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult { tools: self.tool_router.list_all(), next_cursor: None })
    }
    async fn call_tool(&self, request: CallToolRequestParam, context: RequestContext<rmcp::RoleServer>) -> Result<CallToolResult, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}
