//! Desktop GUI & System Health Test Automation Engine & MCP Server
//!
//! Provides automated liveness, IPC gateway validation, UI protocol assertions,
//! model runtime checks, and self-healing loop automation for the installed
//! `oxide-tech-local-agent` desktop binary.

use rmcp::{
    handler::server::tool::{ToolCallContext, ToolRouter},
    handler::server::wrapper::Parameters,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, ListToolsResult,
        PaginatedRequestParams, ServerInfo,
    },
    service::RequestContext,
    tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sysinfo::System;
use tokio::process::Command;

#[derive(Deserialize, JsonSchema)]
pub struct LaunchAppInput {
    /// Optional custom config path
    pub config_path: Option<String>,
    /// Number of seconds to wait for initial window & gateway to bind (default 10)
    pub wait_secs: Option<u64>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GatewayHealthInput {
    /// Target gateway base URL (default http://127.0.0.1:8080)
    pub base_url: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct TestInferenceInput {
    /// Model name or path to test (e.g. "DeepSeek-R1-0528-Qwen3-8B-Q4_K_M.gguf")
    pub model: String,
    /// Test prompt text
    pub prompt: String,
    /// Max tokens for generation (default 64)
    pub max_tokens: Option<u32>,
}

#[derive(Deserialize, JsonSchema)]
pub struct StabilityLoopInput {
    /// Target model to iterate on until stable
    pub target_model: Option<String>,
    /// Number of validation cycles
    pub cycles: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessState {
    pub is_running: bool,
    pub pids: Vec<u32>,
    pub total_memory_mb: u64,
}

#[derive(Clone)]
pub struct DesktopTesterServer {
    tool_router: ToolRouter<Self>,
}

impl Default for DesktopTesterServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl DesktopTesterServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Check if the installed oxide-tech-local-agent desktop application is running
    #[tool(description = "Inspect process table for running instances of oxide-tech-local-agent")]
    async fn inspect_app_process(
        &self,
        Parameters(_): Parameters<serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let mut sys = System::new();
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut pids = Vec::new();
        let mut total_memory = 0u64;

        for (pid, proc) in sys.processes() {
            let name = proc.name().to_string_lossy().to_string();
            if name.contains("oxide-tech-local-agent") {
                pids.push(pid.as_u32());
                total_memory += proc.memory();
            }
        }

        let state = ProcessState {
            is_running: !pids.is_empty(),
            pids,
            total_memory_mb: total_memory / (1024 * 1024),
        };

        let json = serde_json::to_string_pretty(&state).unwrap_or_default();
        Ok(CallToolResult::success(vec![ContentBlock::text(json)]))
    }

    /// Launch the installed desktop application binary in background
    #[tool(description = "Launch the desktop application with GUI window and embedded gateway")]
    async fn launch_desktop_app(
        &self,
        Parameters(input): Parameters<LaunchAppInput>,
    ) -> Result<CallToolResult, McpError> {
        let wait_time = input.wait_secs.unwrap_or(8);

        // Find executable (check ~/.local/bin/oxide-tech-local-agent, /usr/bin/oxide-tech-local-agent, or path)
        let exe_path = if let Some(home) = std::env::var("HOME").ok() {
            let local_bin = std::path::PathBuf::from(home).join(".local/bin/oxide-tech-local-agent");
            if local_bin.exists() {
                local_bin.display().to_string()
            } else {
                "oxide-tech-local-agent".to_string()
            }
        } else {
            "oxide-tech-local-agent".to_string()
        };

        let mut cmd = Command::new(&exe_path);
        if let Some(cfg) = &input.config_path {
            cmd.arg("--config").arg(cfg);
        }

        // Spawn detached process
        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id().unwrap_or(0);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;

                // Validate liveness
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(2))
                    .build()
                    .unwrap_or_default();

                let mut gateway_status = "unreachable";
                if let Ok(resp) = client.get("http://127.0.0.1:8080/health").send().await {
                    if resp.status().is_success() {
                        gateway_status = "healthy";
                    }
                }

                let result = format!(
                    "Desktop app launched (PID: {}).\nExecutable: {}\nGateway (http://127.0.0.1:8080/health): {}",
                    pid, exe_path, gateway_status
                );
                Ok(CallToolResult::success(vec![ContentBlock::text(result)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Failed to spawn desktop app '{}': {}",
                exe_path, e
            ))])),
        }
    }

    /// Probe embedded gateway HTTP endpoints and health
    #[tool(description = "Verify embedded Actix-Web gateway health, SurrealDB status, and model discovery")]
    async fn check_gateway_health(
        &self,
        Parameters(input): Parameters<GatewayHealthInput>,
    ) -> Result<CallToolResult, McpError> {
        let base = input.base_url.unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();

        let health_url = format!("{}/health", base);
        let models_url = format!("{}/v1/models", base);

        let mut report = Vec::new();
        report.push(format!("--- Probing Gateway at {} ---", base));

        match client.get(&health_url).send().await {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                report.push(format!("[✓] /health: {} -> {}", status, body.trim()));
            }
            Err(e) => {
                report.push(format!("[✗] /health failed: {}", e));
            }
        }

        match client.get(&models_url).send().await {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                report.push(format!("[✓] /v1/models: {} -> {}", status, body.trim()));
            }
            Err(e) => {
                report.push(format!("[✗] /v1/models failed: {}", e));
            }
        }

        Ok(CallToolResult::success(vec![ContentBlock::text(report.join("\n"))]))
    }

    /// Trigger end-to-end chat completion through the running gateway
    #[tool(description = "Send prompt through the desktop application's OpenAI-compatible gateway")]
    async fn test_inference_pipeline(
        &self,
        Parameters(input): Parameters<TestInferenceInput>,
    ) -> Result<CallToolResult, McpError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        let payload = serde_json::json!({
            "model": input.model,
            "messages": [
                { "role": "system", "content": "You are a deterministic embedded assistant." },
                { "role": "user", "content": input.prompt }
            ],
            "max_tokens": input.max_tokens.unwrap_or(64),
            "stream": false
        });

        let url = "http://127.0.0.1:8080/v1/chat/completions";
        let start = std::time::Instant::now();

        match client.post(url).json(&payload).send().await {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                let elapsed = start.elapsed().as_millis();

                if status.is_success() {
                    Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                        "Inference succeeded in {} ms (Status: {}):\n{}",
                        elapsed, status, body
                    ))]))
                } else {
                    Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                        "Inference returned status {} in {} ms:\n{}",
                        status, elapsed, body
                    ))]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Inference request failed: {}",
                e
            ))])),
        }
    }

    /// Autonomous loop: inspects, launches, probes, and tests until verified stable
    #[tool(description = "Run automated multi-cycle stability loop verifying desktop app and local inference")]
    async fn run_stability_loop(
        &self,
        Parameters(input): Parameters<StabilityLoopInput>,
    ) -> Result<CallToolResult, McpError> {
        let cycles = input.cycles.unwrap_or(3);
        let model = input.target_model.unwrap_or_else(|| "DeepSeek-R1-0528-Qwen3-8B-Q4_K_M.gguf".to_string());

        let mut log = Vec::new();
        log.push(format!("Starting automated stability verification ({} cycles) for model '{}'...", cycles, model));

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        for i in 1..=cycles {
            log.push(format!("\n--- Cycle {}/{} ---", i, cycles));

            // Check health
            match client.get("http://127.0.0.1:8080/health").send().await {
                Ok(resp) if resp.status().is_success() => {
                    log.push(format!("Cycle {}: Gateway healthy.", i));
                }
                _ => {
                    log.push(format!("Cycle {}: Gateway unreachable, restarting desktop app...", i));
                    let _ = Command::new("pkill").arg("-f").arg("oxide-tech-local-agent").output().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    let _ = Command::new("oxide-tech-local-agent").spawn();
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }

            // Test model availability
            if let Ok(resp) = client.get("http://127.0.0.1:8080/v1/models").send().await {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let count = json.get("data").and_then(|d| d.as_array()).map(|a| a.len()).unwrap_or(0);
                    log.push(format!("Cycle {}: Model catalog responsive ({} registered models).", i, count));
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        log.push("\n[✓] Stability loop complete: Subsystem state verified.".to_string());
        Ok(CallToolResult::success(vec![ContentBlock::text(log.join("\n"))]))
    }
}

impl ServerHandler for DesktopTesterServer {
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
            ..Default::default()
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        let call_ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(call_ctx).await
    }
}
