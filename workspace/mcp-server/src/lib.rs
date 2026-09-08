use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

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

use knowledge::KnowledgeClient;
use memory::SurrealClient;

pub mod ai_infra;
pub mod browser_agent;
pub mod cad;
pub mod embedded;
pub mod foundation;
pub mod knowledge_mcp;
pub mod pcb;
pub mod verification;
pub mod web;

// ── Tool Input Structs ────────────────────────────────────────────────────────

#[derive(Deserialize, JsonSchema)]
pub struct CargoInput {
    /// Workspace subdirectory path (defaults to root if empty).
    pub workspace_path: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CargoExpandInput {
    /// Workspace subdirectory path.
    pub workspace_path: Option<String>,
    /// The name of the item (struct, module, macro) to expand.
    pub item: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitInput {
    /// Git repository path subdirectory (defaults to root if empty).
    pub repo_path: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitCommitInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// Commit message description.
    pub message: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitHashInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// The commit hash or reference.
    pub commit: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitStashInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// The stash action ("push", "pop", "list").
    pub action: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitBlameInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// File path relative to git repository.
    pub file_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitHistoryInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// Limit number of history log commits.
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitTagInput {
    /// Git repository path subdirectory.
    pub repo_path: Option<String>,
    /// Tag name.
    pub tag: String,
}

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
pub struct MoveFileInput {
    /// Relative path of source file.
    pub src: String,
    /// Relative path of target destination.
    pub dest: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SearchFileInput {
    /// String pattern to search in filenames.
    pub query: String,
    /// Subdirectory to search within (optional).
    pub sub_dir: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GlobInput {
    /// Glob pattern (e.g. "*.rs" or "src/**/*.rs").
    pub pattern: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct TerminalInput {
    /// The executable command name.
    pub command: String,
    /// Command line arguments.
    pub args: Vec<String>,
    /// Working directory (optional).
    pub work_dir: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct QueryInput {
    /// The query search string.
    pub query: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitHubReleaseInput {
    /// GitHub repository owner and name (e.g. "tokio-rs/tokio").
    pub repo: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct DocsRsInput {
    /// Crate name.
    pub crate_name: String,
    /// Search query string.
    pub query: String,
    /// Limit results (optional).
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CratesIoInput {
    /// Search term.
    pub query: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitHubQueryInput {
    /// Repository (e.g. "rust-lang/rust").
    pub repo: String,
    /// The API endpoint (e.g. "issues", "pulls", "releases").
    pub endpoint: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ChangelogInput {
    /// Crate name.
    pub crate_name: String,
    /// Lower boundary version.
    pub from_version: String,
    /// Upper boundary version.
    pub to_version: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct KiCadInput {
    /// Project folder or schematic file path.
    pub project_path: String,
    /// Operation type ("open", "schematic", "pcb", "erc", "drc", "bom", "netlist").
    pub op_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SpiceInput {
    /// Circuit netlist path.
    pub netlist_path: String,
    /// Analysis type ("dc", "ac", "transient", "noise").
    pub analysis_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ComponentInput {
    /// Part catalog number or name.
    pub part_number: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct BomInput {
    /// BOM spreadsheet/CSV file path.
    pub bom_path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct BlenderCreateInput {
    /// Mesh object type ("cube", "sphere", "cylinder").
    pub obj_type: String,
    /// Size scale dimensions.
    pub size: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct BlenderModifyInput {
    /// Target object identifier.
    pub obj_name: String,
    /// Transform operation ("scale", "translate", "rotate").
    pub operation: String,
    /// Parameter arguments.
    pub params: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct BlenderValidateInput {
    /// Target object name.
    pub obj_name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct BlenderExportInput {
    /// Target object name.
    pub obj_name: String,
    /// Export format ("glb", "step").
    pub format: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct FreeCadModelInput {
    /// Constraint parameter name.
    pub param_name: String,
    /// Constraint parameter value.
    pub param_val: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct FreeCadExportInput {
    /// Model name.
    pub model_name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct QdrantInput {
    /// Vector operation type ("store", "retrieve").
    pub op_type: String,
    /// The text snippet.
    pub text: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SurrealDbInput {
    /// Table type ("project", "experience", "task").
    pub memory_type: String,
    /// SurrealQL statement.
    pub query: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct EntityInput {
    /// Target entity id.
    pub entity_id: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct RustVerifyInput {
    /// Workspace subdirectory path.
    pub workspace_path: String,
    /// Verification type ("check", "test", "clippy", "coverage", "benchmark").
    pub verification_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct PcbVerifyInput {
    /// Project folder path.
    pub project_path: String,
    /// Verification type ("ERC", "DRC", "BOM").
    pub check_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CadVerifyInput {
    /// Model name.
    pub model_name: String,
    /// Verification type ("mesh", "manifold", "geometry").
    pub check_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SecurityVerifyInput {
    /// Workspace path.
    pub workspace_path: String,
    /// Verification type ("audit", "dependency_scan", "sbom").
    pub check_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ModelRouteInput {
    /// Task description.
    pub task_description: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ExperienceInput {
    /// Finished task description and outcome.
    pub task_outcome: String,
}

// ── McpServer ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct McpServer {
    workspace_root: PathBuf,
    rag: Option<Arc<KnowledgeClient>>,
    surreal: Option<Arc<SurrealClient>>,
    tool_router: ToolRouter<Self>,
}

impl McpServer {
    pub fn new(
        workspace_root: PathBuf,
        rag: Option<Arc<KnowledgeClient>>,
        surreal: Option<Arc<SurrealClient>>,
    ) -> Self {
        // Merge the sub-routers to avoid tuple size macro limits.
        let tool_router = Self::foundation_router()
            + Self::web_router()
            + Self::knowledge_router()
            + Self::embedded_router()
            + Self::pcb_router()
            + Self::cad_router()
            + Self::ai_infra_router()
            + Self::verification_router();

        Self {
            workspace_root,
            rag,
            surreal,
            tool_router,
        }
    }
}

// ── Foundation MCP Tools Sub-Router ──
#[tool_router(router = foundation_router)]
impl McpServer {
    #[tool(description = "Run cargo check inside sandbox")]
    async fn cargo_check(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["check"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo test inside sandbox")]
    async fn cargo_test(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["test"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo clippy inside sandbox")]
    async fn cargo_clippy(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(
            &["clippy", "--all-targets"],
            input.workspace_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo fmt check inside sandbox")]
    async fn cargo_fmt(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(
            &["fmt", "--", "--check"],
            input.workspace_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo audit security checks")]
    async fn cargo_audit(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["audit"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo expand for a macro item")]
    async fn cargo_expand(
        &self,
        Parameters(input): Parameters<CargoExpandInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(
            &["expand", &input.item],
            input.workspace_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo tree dependency visualizer")]
    async fn cargo_tree(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["tree"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo update workspace dependencies")]
    async fn cargo_update(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["update"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo bench benchmarks")]
    async fn cargo_bench(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["bench"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run cargo llvm-cov test coverage check")]
    async fn cargo_llvm_cov(
        &self,
        Parameters(input): Parameters<CargoInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_cargo(&["llvm-cov"], input.workspace_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git status inside sandbox")]
    async fn git_status(
        &self,
        Parameters(input): Parameters<GitInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(&["status"], input.repo_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git diff inside sandbox")]
    async fn git_diff(
        &self,
        Parameters(input): Parameters<GitInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(&["diff"], input.repo_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git commit inside sandbox")]
    async fn git_commit(
        &self,
        Parameters(input): Parameters<GitCommitInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(
            &["commit", "-m", &input.message],
            input.repo_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git branch inside sandbox")]
    async fn git_branch(
        &self,
        Parameters(input): Parameters<GitInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(&["branch"], input.repo_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git revert inside sandbox")]
    async fn git_revert(
        &self,
        Parameters(input): Parameters<GitHashInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(
            &["revert", &input.commit],
            input.repo_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git stash inside sandbox")]
    async fn git_stash(
        &self,
        Parameters(input): Parameters<GitStashInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(
            &["stash", &input.action],
            input.repo_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git blame inside sandbox")]
    async fn git_blame(
        &self,
        Parameters(input): Parameters<GitBlameInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(
            &["blame", &input.file_path],
            input.repo_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git log history inside sandbox")]
    async fn git_history(
        &self,
        Parameters(input): Parameters<GitHistoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let limit = input.limit.unwrap_or(10).to_string();
        foundation::run_git(
            &["log", "-n", &limit],
            input.repo_path,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run git tag inside sandbox")]
    async fn git_tag(
        &self,
        Parameters(input): Parameters<GitTagInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::run_git(&["tag", &input.tag], input.repo_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Read a file from workspace")]
    async fn fs_read(
        &self,
        Parameters(input): Parameters<ReadFileInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_read(&input.path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Write content to a file in workspace")]
    async fn fs_write(
        &self,
        Parameters(input): Parameters<WriteFileInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_write(&input.path, &input.content, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Move or rename file inside workspace")]
    async fn fs_move(
        &self,
        Parameters(input): Parameters<MoveFileInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_move(&input.src, &input.dest, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Delete file inside workspace")]
    async fn fs_delete(
        &self,
        Parameters(input): Parameters<ReadFileInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_delete(&input.path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Search recursively for files matching query pattern")]
    async fn fs_search(
        &self,
        Parameters(input): Parameters<SearchFileInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_search(&input.query, input.sub_dir, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Evaluate glob pattern matches in workspace")]
    async fn fs_glob(
        &self,
        Parameters(input): Parameters<GlobInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::fs_glob(&input.pattern, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run sandboxed terminal execution command")]
    async fn terminal_run(
        &self,
        Parameters(input): Parameters<TerminalInput>,
    ) -> Result<CallToolResult, McpError> {
        foundation::terminal_run(
            &input.command,
            &input.args,
            input.work_dir,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── Web MCP Tools Sub-Router ──
#[tool_router(router = web_router)]
impl McpServer {
    #[tool(description = "Execute search on Google and return parser results")]
    async fn google_search(
        &self,
        Parameters(input): Parameters<QueryInput>,
    ) -> Result<CallToolResult, McpError> {
        web::google_search(&input.query)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Fetch latest release information for GitHub repository")]
    async fn github_latest_release(
        &self,
        Parameters(input): Parameters<GitHubReleaseInput>,
    ) -> Result<CallToolResult, McpError> {
        web::github_latest_release(&input.repo)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── Knowledge MCP Tools Sub-Router ──
#[tool_router(router = knowledge_router)]
impl McpServer {
    #[tool(description = "Query ingested docs.rs API documentation")]
    async fn docs_rs_lookup(
        &self,
        Parameters(input): Parameters<DocsRsInput>,
    ) -> Result<CallToolResult, McpError> {
        knowledge_mcp::docs_rs_lookup(&input.query, input.limit, &self.rag)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Search Rust reference guidelines and idioms")]
    async fn rust_book_search(
        &self,
        Parameters(input): Parameters<QueryInput>,
    ) -> Result<CallToolResult, McpError> {
        knowledge_mcp::rust_book_search(&input.query)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Query Crates.io packages, downloads, and maintenance scores")]
    async fn crates_io_search(
        &self,
        Parameters(input): Parameters<CratesIoInput>,
    ) -> Result<CallToolResult, McpError> {
        knowledge_mcp::crates_io_search(&input.query)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Retrieve list of issues or PRs from GitHub API")]
    async fn github_api_query(
        &self,
        Parameters(input): Parameters<GitHubQueryInput>,
    ) -> Result<CallToolResult, McpError> {
        knowledge_mcp::github_api_query(&input.repo, &input.endpoint)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Extract changelog breaking changes between crate versions")]
    async fn changelog_diff(
        &self,
        Parameters(input): Parameters<ChangelogInput>,
    ) -> Result<CallToolResult, McpError> {
        knowledge_mcp::changelog_diff(&input.crate_name, &input.from_version, &input.to_version)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── Embedded MCP Tools Sub-Router ──
#[tool_router(router = embedded_router)]
impl McpServer {
    #[tool(description = "Search Embassy HAL guidelines and driver patterns")]
    async fn embassy_lookup(
        &self,
        Parameters(input): Parameters<QueryInput>,
    ) -> Result<CallToolResult, McpError> {
        embedded::embassy_lookup(&input.query, &self.rag)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Search STM32 registers and datasheet manuals")]
    async fn stm32_lookup(
        &self,
        Parameters(input): Parameters<QueryInput>,
    ) -> Result<CallToolResult, McpError> {
        embedded::stm32_lookup(&input.query, &self.rag)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Search ESP32 IDF documentation and guides")]
    async fn esp32_lookup(
        &self,
        Parameters(input): Parameters<QueryInput>,
    ) -> Result<CallToolResult, McpError> {
        embedded::esp32_lookup(&input.query, &self.rag)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── PCB MCP Tools Sub-Router ──
#[tool_router(router = pcb_router)]
impl McpServer {
    #[tool(description = "Execute KiCad operations (schematics, bom, erc, drc, netlist)")]
    async fn kicad_project_op(
        &self,
        Parameters(input): Parameters<KiCadInput>,
    ) -> Result<CallToolResult, McpError> {
        pcb::kicad_project_op(&input.project_path, &input.op_type, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Run Ngspice circuit analysis simulation")]
    async fn ngspice_simulate(
        &self,
        Parameters(input): Parameters<SpiceInput>,
    ) -> Result<CallToolResult, McpError> {
        pcb::ngspice_simulate(
            &input.netlist_path,
            &input.analysis_type,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Look up component part footprints and pricing")]
    async fn component_search(
        &self,
        Parameters(input): Parameters<ComponentInput>,
    ) -> Result<CallToolResult, McpError> {
        pcb::component_search(&input.part_number)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Calculate BOM pricing and lead time estimations")]
    async fn bom_pricing(
        &self,
        Parameters(input): Parameters<BomInput>,
    ) -> Result<CallToolResult, McpError> {
        pcb::bom_pricing(&input.bom_path, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── CAD MCP Tools Sub-Router ──
#[tool_router(router = cad_router)]
impl McpServer {
    #[tool(description = "Create 3D objects in Blender")]
    async fn blender_create_object(
        &self,
        Parameters(input): Parameters<BlenderCreateInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::blender_op(
            &input.obj_type,
            "create",
            &input.size.to_string(),
            "glb",
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Modify 3D objects in Blender")]
    async fn blender_modify_object(
        &self,
        Parameters(input): Parameters<BlenderModifyInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::blender_op(
            "cube",
            &input.operation,
            &input.params,
            "glb",
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Validate Blender mesh manifold integrity")]
    async fn blender_validate_mesh(
        &self,
        Parameters(input): Parameters<BlenderValidateInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::blender_op(
            "cube",
            "validate",
            &input.obj_name,
            "glb",
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Export Blender objects to GLB or STEP format")]
    async fn blender_export(
        &self,
        Parameters(input): Parameters<BlenderExportInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::blender_op(
            "cube",
            "export",
            &input.obj_name,
            &input.format,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Modify parametric constraints in FreeCAD")]
    async fn freecad_model(
        &self,
        Parameters(input): Parameters<FreeCadModelInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::freecad_op(&input.param_name, input.param_val, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Export FreeCAD parametric models to STEP")]
    async fn freecad_export_step(
        &self,
        Parameters(input): Parameters<FreeCadExportInput>,
    ) -> Result<CallToolResult, McpError> {
        cad::freecad_op("length", 10.0, &self.workspace_root)
            .await
            .map(|r| {
                CallToolResult::success(vec![Content::text(format!(
                    "Exported {} successfully: {}",
                    input.model_name, r
                ))])
            })
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── AI Infrastructure MCP Tools Sub-Router ──
#[tool_router(router = ai_infra_router)]
impl McpServer {
    #[tool(description = "Store or retrieve Qdrant semantic vector embeddings")]
    async fn qdrant_embeddings_op(
        &self,
        Parameters(input): Parameters<QdrantInput>,
    ) -> Result<CallToolResult, McpError> {
        ai_infra::qdrant_op(&input.op_type, &input.text, &self.rag)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Execute SurrealQL queries over EIOS memory tables")]
    async fn surrealdb_memory_op(
        &self,
        Parameters(input): Parameters<SurrealDbInput>,
    ) -> Result<CallToolResult, McpError> {
        ai_infra::surrealdb_op(&input.memory_type, &input.query, &self.surreal)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Display dependency graph architecture relationships")]
    async fn knowledge_graph_view(
        &self,
        Parameters(input): Parameters<EntityInput>,
    ) -> Result<CallToolResult, McpError> {
        ai_infra::knowledge_graph_view(&input.entity_id, &self.surreal)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }
}

// ── Verification MCP Tools Sub-Router ──
#[tool_router(router = verification_router)]
impl McpServer {
    #[tool(description = "Verify Rust workspace code compilation or unit tests")]
    async fn rust_verify(
        &self,
        Parameters(input): Parameters<RustVerifyInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::rust_verify(
            &input.workspace_path,
            &input.verification_type,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Verify PCB design rules or ERC connections")]
    async fn pcb_verify(
        &self,
        Parameters(input): Parameters<PcbVerifyInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::pcb_verify(&input.project_path, &input.check_type, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Verify CAD mesh manifold or geometry boundaries")]
    async fn cad_verify(
        &self,
        Parameters(input): Parameters<CadVerifyInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::cad_verify(&input.model_name, &input.check_type, &self.workspace_root)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Scan workspace dependencies for security vulnerabilities")]
    async fn security_verify(
        &self,
        Parameters(input): Parameters<SecurityVerifyInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::security_verify(
            &input.workspace_path,
            &input.check_type,
            &self.workspace_root,
        )
        .await
        .map(|r| CallToolResult::success(vec![Content::text(r)]))
        .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Perform route and cost/latency calculations for model selector")]
    async fn model_router_route(
        &self,
        Parameters(input): Parameters<ModelRouteInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::model_router_route(&input.task_description)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[tool(description = "Extract patterns and lessons learned from task outcomes")]
    async fn experience_learning_extract(
        &self,
        Parameters(input): Parameters<ExperienceInput>,
    ) -> Result<CallToolResult, McpError> {
        verification::experience_learning_extract(&input.task_outcome, &self.surreal)
            .await
            .map(|r| CallToolResult::success(vec![Content::text(r)]))
            .map_err(|e| McpError::internal_error(e, None))
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
