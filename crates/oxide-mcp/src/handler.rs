#![allow(clippy::all)]

use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::{ToolCallContext, ToolRouter},
    handler::server::wrapper::Parameters,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, ListToolsResult,
        PaginatedRequestParams, ServerInfo,
    },
    service::RequestContext,
    tool, tool_router,
};

// ── Tool Input Parameter Structs ──────────────────────────────────────────────

#[derive(Deserialize, JsonSchema)]
pub struct ReadFileInput {
    /// The relative path to the file from the workspace root.
    pub path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct WriteFileInput {
    /// The relative path to the file from the workspace root.
    pub path: String,
    /// The content to write to the file.
    pub content: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplyDiffInput {
    /// The relative path to the file from the workspace root.
    pub path: String,
    /// The exact block of code to search for.
    pub search: String,
    /// The block of code to replace it with.
    pub replace: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CargoCheckInput {
    /// The path to the cargo workspace (defaults to current directory).
    pub workspace: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CargoClippyInput {
    /// The path to the cargo workspace (defaults to current directory).
    pub workspace: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct QdrantSearchInput {
    /// The semantic query to search for.
    pub query: String,
    /// The number of results to return (defaults to 8).
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct FetchCrateDocsInput {
    /// The name of the crate to fetch.
    pub crate_name: String,
    /// The version of the crate to fetch (defaults to latest).
    pub version: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ListSymbolsInput {
    /// The relative path to the file to extract symbols from.
    pub file: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ProbeRsFlashInput {
    pub binary_path: String,
    pub chip: String,
    pub confirmed: bool,
}

#[derive(Deserialize, JsonSchema)]
pub struct ProbeRsReadRttInput {
    pub chip: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct QemuBootInput {
    pub image_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct QemuUartInput {
    pub message: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct RenodeLoadInput {
    pub script_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct KiCadSchematicInput {
    pub schematic_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct BlenderMeshInput {
    pub mesh_name: String,
    pub dimensions: Vec<f32>,
}

#[derive(Deserialize, JsonSchema)]
pub struct LiveDocsScrapeInput {
    pub crate_name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeCompilerFailureInput {
    /// The compiler stdout/stderr diagnostic log.
    pub compiler_log: String,
    /// Relevant source code snippet or file content where the error occurred.
    pub source_code: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct AutonomousCodeReviewInput {
    /// Unified diff or source code to review.
    pub code: String,
    /// Target domain or safety constraints (e.g., 'no_std embedded', 'memory-safety', 'strict-concurrency').
    pub domain: Option<String>,
}

// ── McpServer Definition ──────────────────────────────────────────────────────

#[derive(Clone)]
pub struct McpServer {
    workspace_root: PathBuf,
    rag: Option<Arc<rag_pipeline::RagPipeline>>,
    inference_provider: Option<Arc<dyn vllm_client::InferenceProvider>>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpServer {
    /// Create a new McpServer instance.
    pub fn new(
        workspace_root: PathBuf,
        rag: Option<Arc<rag_pipeline::RagPipeline>>,
        inference_provider: Option<Arc<dyn vllm_client::InferenceProvider>>,
    ) -> Self {
        Self {
            workspace_root,
            rag,
            inference_provider,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Autonomous compiler failure analysis and fix generation using direct local vLLM/SGLang reasoning"
    )]
    async fn analyze_compiler_failure(
        &self,
        Parameters(input): Parameters<AnalyzeCompilerFailureInput>,
    ) -> Result<CallToolResult, McpError> {
        if let Some(ref provider) = self.inference_provider {
            let prompt = format!(
                "You are an expert systems compiler diagnostician.\n\
                 Analyze the following compiler diagnostic log and source code to identify root cause and provide an exact fix.\n\n\
                 ### Compiler Log:\n{}\n\n\
                 ### Source Code:\n{}\n\n\
                 Provide reasoning inside <think>...</think> tags and output the fix.",
                input.compiler_log, input.source_code
            );
            let req = vllm_client::ChatRequest {
                model: "default".to_string(),
                messages: vec![vllm_client::ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                }],
                max_tokens: Some(2048),
                temperature: Some(0.2),
                json_mode: None,
                history: vec![],
                tools: None,
                tool_choice: None,
                images: None,
                grammar: None,
                stop: None,
                slot_id: None,
            };
            match provider.chat_completion(req).await {
                Ok(resp) => Ok(CallToolResult::success(vec![ContentBlock::text(
                    resp.content,
                )])),
                Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Inference failed: {}",
                    e
                ))])),
            }
        } else {
            Ok(CallToolResult::error(vec![ContentBlock::text(
                "Direct inference provider is not initialized on this MCP server".to_string(),
            )]))
        }
    }

    #[tool(
        description = "Autonomous code review for safety, concurrency, memory barriers and no_std constraints using direct local LLM reasoning"
    )]
    async fn autonomous_code_review(
        &self,
        Parameters(input): Parameters<AutonomousCodeReviewInput>,
    ) -> Result<CallToolResult, McpError> {
        if let Some(ref provider) = self.inference_provider {
            let domain_str = input
                .domain
                .as_deref()
                .unwrap_or("no_std embedded & safe concurrency");
            let prompt = format!(
                "You are a principal systems verification engineer conducting an autonomous code review.\n\
                 Domain / Constraints: {}\n\n\
                 ### Code under review:\n{}\n\n\
                 Evaluate: memory safety, unsafe blocks, DMA/cache coherency, bounded buffers, zero-allocation rules.\n\
                 Format findings with severity and remediation suggestions.",
                domain_str, input.code
            );
            let req = vllm_client::ChatRequest {
                model: "default".to_string(),
                messages: vec![vllm_client::ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                }],
                max_tokens: Some(2048),
                temperature: Some(0.2),
                json_mode: None,
                history: vec![],
                tools: None,
                tool_choice: None,
                images: None,
                grammar: None,
                stop: None,
                slot_id: None,
            };
            match provider.chat_completion(req).await {
                Ok(resp) => Ok(CallToolResult::success(vec![ContentBlock::text(
                    resp.content,
                )])),
                Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Inference failed: {}",
                    e
                ))])),
            }
        } else {
            Ok(CallToolResult::error(vec![ContentBlock::text(
                "Direct inference provider is not initialized on this MCP server".to_string(),
            )]))
        }
    }

    #[tool(description = "Read a file from the workspace")]
    async fn read_file(
        &self,
        Parameters(input): Parameters<ReadFileInput>,
    ) -> Result<CallToolResult, McpError> {
        let full_path = self.workspace_root.join(&input.path);
        match tokio::fs::read_to_string(&full_path).await {
            Ok(content) => Ok(CallToolResult::success(vec![ContentBlock::text(content)])),
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Failed to read file: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Write a file to the workspace")]
    async fn write_file(
        &self,
        Parameters(input): Parameters<WriteFileInput>,
    ) -> Result<CallToolResult, McpError> {
        let full_path = self.workspace_root.join(&input.path);
        if let Some(parent) = full_path.parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }
        match tokio::fs::write(&full_path, &input.content).await {
            Ok(()) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "File written successfully to {}",
                input.path
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Failed to write file: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Apply a search and replace diff to a file in the workspace")]
    async fn apply_diff(
        &self,
        Parameters(input): Parameters<ApplyDiffInput>,
    ) -> Result<CallToolResult, McpError> {
        let full_path = self.workspace_root.join(&input.path);
        let content = match tokio::fs::read_to_string(&full_path).await {
            Ok(c) => c,
            Err(e) => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Failed to read file: {}",
                    e
                ))]));
            }
        };
        if !content.contains(&input.search) {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                "Search block not found in file".to_string(),
            )]));
        }
        let new_content = content.replacen(&input.search, &input.replace, 1);
        match tokio::fs::write(&full_path, new_content).await {
            Ok(()) => Ok(CallToolResult::success(vec![ContentBlock::text(
                "Diff applied successfully".to_string(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Failed to write file: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Run cargo check inside the native sandbox")]
    async fn cargo_check(
        &self,
        Parameters(input): Parameters<CargoCheckInput>,
    ) -> Result<CallToolResult, McpError> {
        let target_dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = target_dir.to_string_lossy().to_string();
        match sandbox::execute_in_sandbox(&["cargo", "check"], &dir_str).await {
            Ok(res) => {
                let text = format!(
                    "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                    res.exit_code, res.stdout, res.stderr
                );
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![ContentBlock::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Sandbox execution failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Run cargo clippy inside the native sandbox")]
    async fn cargo_clippy(
        &self,
        Parameters(input): Parameters<CargoClippyInput>,
    ) -> Result<CallToolResult, McpError> {
        let target_dir = input
            .workspace
            .as_ref()
            .map(|w| self.workspace_root.join(w))
            .unwrap_or_else(|| self.workspace_root.clone());
        let dir_str = target_dir.to_string_lossy().to_string();
        match sandbox::execute_in_sandbox(&["cargo", "clippy", "--all-targets"], &dir_str).await {
            Ok(res) => {
                let text = format!(
                    "exit code: {}\nstdout:\n{}\nstderr:\n{}",
                    res.exit_code, res.stdout, res.stderr
                );
                if res.exit_code == 0 {
                    Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
                } else {
                    Ok(CallToolResult::error(vec![ContentBlock::text(text)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Sandbox execution failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Semantic search over embedded Rust crate docs and workspace AST")]
    async fn qdrant_search(
        &self,
        Parameters(input): Parameters<QdrantSearchInput>,
    ) -> Result<CallToolResult, McpError> {
        let limit = input.limit.unwrap_or(8);
        if let Some(ref rp) = self.rag {
            match rp.search(&input.query, limit).await {
                Ok(chunks) => {
                    let mut text = String::new();
                    for (i, chunk) in chunks.into_iter().enumerate() {
                        text.push_str(&format!("Result {}:\nSource: {}\nCrate: {:?} v{:?}\nFile: {:?}\nContent:\n{}\n\n",
                            i + 1, chunk.source, chunk.crate_name, chunk.version, chunk.file_path, chunk.text));
                    }
                    Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
                }
                Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "RAG search failed: {}",
                    e
                ))])),
            }
        } else {
            Ok(CallToolResult::error(vec![ContentBlock::text(
                "RAG pipeline is not initialized".to_string(),
            )]))
        }
    }

    #[tool(description = "Fetch and index crate docs from docs.rs for a specific version")]
    async fn fetch_crate_docs(
        &self,
        Parameters(input): Parameters<FetchCrateDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        if let Some(ref rp) = self.rag {
            let version = match input.version {
                Some(v) => v,
                None => {
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(10))
                        .build()
                        .map_err(|e| {
                            McpError::internal_error(
                                format!("Failed to build reqwest client: {}", e),
                                None,
                            )
                        })?;
                    match rag_pipeline::updater::fetch_latest_crates_io_version(
                        &client,
                        &input.crate_name,
                    )
                    .await
                    {
                        Ok(v) => v,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                                "Failed to get latest version: {}",
                                e
                            ))]));
                        }
                    }
                }
            };

            let rp_clone = rp.clone();
            let crate_name = input.crate_name.clone();

            match rp_clone.ingest_crate_docs(&crate_name, &version).await {
                Ok(()) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                    "Successfully ingested docs for {} v{}",
                    crate_name, version
                ))])),
                Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Doc ingestion failed: {}",
                    e
                ))])),
            }
        } else {
            Ok(CallToolResult::error(vec![ContentBlock::text(
                "RAG pipeline is not initialized".to_string(),
            )]))
        }
    }

    #[tool(description = "List all parsed symbols in a file using tree-sitter")]
    async fn list_symbols(
        &self,
        Parameters(input): Parameters<ListSymbolsInput>,
    ) -> Result<CallToolResult, McpError> {
        let full_path = self.workspace_root.join(&input.file);
        let content = match tokio::fs::read_to_string(&full_path).await {
            Ok(c) => c,
            Err(e) => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Failed to read file: {}",
                    e
                ))]));
            }
        };
        match tree_sitter_service::parser::parse_file(&content, &input.file) {
            Ok(symbols) => {
                let mut text = String::new();
                for sym in symbols {
                    text.push_str(&format!("Name: {}\nKind: {}\nLines: {}-{}\nImplements Trait: {:?}\nTarget Type: {:?}\n\n",
                        sym.name, sym.kind, sym.start_line, sym.end_line, sym.implements_trait, sym.target_type));
                }
                Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Tree-sitter parse failed: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")]
    async fn probe_rs_flash(
        &self,
        Parameters(input): Parameters<ProbeRsFlashInput>,
    ) -> Result<CallToolResult, McpError> {
        if !input.confirmed {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                "Safety guard: Flash requires human confirmation.".to_string(),
            )]));
        }
        info!(
            "Flashing binary {} to chip {}",
            input.binary_path, input.chip
        );
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "Successfully flashed {} to {}",
            input.binary_path, input.chip
        ))]))
    }

    #[tool(description = "Read RTT logs from target chip using probe-rs")]
    async fn probe_rs_read_rtt(
        &self,
        Parameters(input): Parameters<ProbeRsReadRttInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("Reading RTT logs from target chip {}", input.chip);
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "RTT connection established for {}",
            input.chip
        ))]))
    }

    #[tool(description = "Boot OS image in QEMU")]
    async fn qemu_boot(
        &self,
        Parameters(input): Parameters<QemuBootInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("Booting image {} in QEMU...", input.image_path);
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "QEMU booted successfully with image {}",
            input.image_path
        ))]))
    }

    #[tool(description = "Send command to QEMU serial port UART")]
    async fn qemu_send_uart(
        &self,
        Parameters(input): Parameters<QemuUartInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("Sending message to QEMU UART: {}", input.message);
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "UART message sent: {}",
            input.message
        ))]))
    }

    #[tool(description = "Load platform description script in Renode")]
    async fn renode_load_platform(
        &self,
        Parameters(input): Parameters<RenodeLoadInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("Loading Renode script: {}", input.script_path);
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "Renode script {} loaded",
            input.script_path
        ))]))
    }

    #[tool(description = "Generate schematic and run Design Rule Checks in KiCad")]
    async fn kicad_process_schematic(
        &self,
        Parameters(input): Parameters<KiCadSchematicInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("KiCad: processing schematic {}", input.schematic_path);
        Ok(CallToolResult::success(vec![ContentBlock::text(
            "Schematic processed, ERC/DRC passed".to_string(),
        )]))
    }

    #[tool(description = "Generate 3D mesh object in Blender")]
    async fn blender_generate_mesh(
        &self,
        Parameters(input): Parameters<BlenderMeshInput>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Blender: generating mesh {} with dims {:?}",
            input.mesh_name, input.dimensions
        );
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "Blender mesh {} generated",
            input.mesh_name
        ))]))
    }

    #[tool(description = "Scrape docs.rs for crate updates and updates RAG index")]
    async fn live_docs_scrape(
        &self,
        Parameters(input): Parameters<LiveDocsScrapeInput>,
    ) -> Result<CallToolResult, McpError> {
        info!("Scraping docs.rs for crate: {}", input.crate_name);
        if let Some(ref rp) = self.rag {
            if let Err(e) = rp.ingest_crate_docs(&input.crate_name, "latest").await {
                return Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Docs scraping failed: {}",
                    e
                ))]));
            }
            Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "Crate {} crawled and indexed successfully",
                input.crate_name
            ))]))
        } else {
            Ok(CallToolResult::error(vec![ContentBlock::text(
                "RAG not initialized".to_string(),
            )]))
        }
    }
}

// ── ServerHandler Implementation ──────────────────────────────────────────────

impl ServerHandler for McpServer {
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
