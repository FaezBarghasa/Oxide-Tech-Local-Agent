# Oxide-Tech Local Agent OS: Changelog

All notable changes to the Oxide-Tech Local Agent OS codebase are documented here.

---

## [v0.9.0-unsloth-cloudroom] - 2026-09-21

This milestone delivers **Unsloth-Grade GPU Kernel Autotuning, Distributed RL Scaling, and Cloudroom Agent Runtime Safety & Process Supervision** across the Oxide-Tech backend.

### Major Upgrades & Enhancements

#### 1. Unsloth-Parity GPU Kernels & Model Architecture (`crates/oxide-kernels` & `crates/model-trainer`)
- **GPU Architecture Autotuning (`autotune.rs`)**: Dynamic SM detection across Nvidia Ampere (SM80/86), Ada (SM89), Hopper (SM90), and Blackwell (SM100/120) with memoized tile sizing ($M, N, K$, warp allocation, stages).
- **HuggingFace AutoModel Hub Loader (`hf_hub.rs`)**: Automatic architecture parsing and sharded `model.safetensors.index.json` weight loader for causal language models.
- **Online RL Fine-Tuning Engine (`rl_engine.rs`)**: Implementations of Direct Preference Optimization (DPO), Odds Ratio Preference Optimization (ORPO), and Group Relative Policy Optimization (GRPO with group advantage normalization, PPO clipping, and KL penalty).
- **Multi-Node ZeRO-3 Parameter & Gradient Sharding (`distributed.rs`)**: Layer-wise `all_gather_parameter` and `reduce_scatter_gradients` across multi-node GPU clusters.

#### 2. DDR5 Host RAM Spillover & AVX-512 SIMD Compression (`crates/model-trainer` & `crates/oxide-kernels`)
- **Hierarchical Memory Tier Manager (`ddr5_offload.rs`)**: Automatic DMA eviction from GPU VRAM to pinned DDR5 host RAM when VRAM headroom drops below $800\text{ MB}$, with zero-copy prefetching back to GPU.
- **AVX-512 Tensor Compression (`avx512_compress.rs`)**: 4x memory bandwidth reduction converting FP32 tensors to INT8 with dynamic scaling factors via AVX-512F/BW SIMD vectorization and CPU runtime fallback.

#### 3. Cloudroom Core Process Supervisor & Runtime Safety (`crates/oxide-security`)
- **Circuit Breaker Resource Gating (`resource_gater.rs`)**: Polling-based disk space ($< 2\text{GB}$ freeze / $> 2.5\text{GB}$ resume hysteresis) and VRAM tripwire admission control integrated into `oxide-gateway` and `AppState`.
- **Idempotent Session State Machine (`session_supervisor.rs`)**: Atomic fsync receipt persistence (`SessionReceipt`), request deduplication, and a strict single-attempt crash recovery guarantee.
- **Secure Stderr Capture & Sanitization (`stderr_sanitizer.rs`)**: 16 KiB bounded ring buffer tail, regex secret scrubbing (OpenAI/HF/GitHub API keys, Bearer tokens), and `0600` root-restricted JSONL diagnostic storage.
- **Bounded Diagnostic Outbox (`bounded_outbox.rs`)**: Non-blocking 1,024-entry channel with Prometheus drop counter tracking and 8 MiB batch log file rotation.
- **Process Tree Containment (`process_containment.rs`)**: Process group (`setpgid`) isolation, 4-second SIGTERM grace period, and guaranteed SIGKILL fallback to eliminate zombie child processes.

---

## [v0.8.0-forge-rust] - 2026-09-19

This milestone delivers the **Universal Polyglot-to-Rust Refactoring Engine (`crates/forge-rust`)**, providing automated AST lifting, semantic restructuring, safe ownership/error mapping, and crate scaffolding for converting arbitrary foreign codebases into idiomatic Rust 2024.

### Major Upgrades & Enhancements

#### 1. Universal Polyglot to Rust Refactoring Engine (`crates/forge-rust`)
- **Multi-Language Frontends**: Implemented dedicated AST and pattern lifters for C/C++, Python, TypeScript/JavaScript, Go, and Generic procedural/OOP source code.
- **Polyglot UIR**: Designed Universal Intermediate Representation (`UirModule`, `UirStruct`, `UirFunction`, `UirTrait`, `UirType`, `UirStmt`, `UirExpr`) capturing cross-language type semantics.
- **Idiomatic Rust Refactoring Pipeline**:
  - `NamingPass`: Normalizes identifiers into Rust casing (`snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`).
  - `OwnershipPass`: Lifts raw pointers and garbage-collected references into safe Rust references (`&`, `&mut`), `Box<T>`, and `Arc<Mutex<T>>`.
  - `ErrorHandlingPass`: Maps nullable types, errno codes, and exceptions into idiomatic `Result<T, E>` and `Option<T>` with `?` operator support.
  - `CompositionPass`: Converts OOP inheritance and method receivers into struct `impl` blocks and traits.
  - `ConcurrencyPass`: Analyzes async/await and goroutines to inject `tokio` dependencies.
- **Syntax Validation & Scaffolding**: Built-in verification via `syn::parse_file` and full `Cargo.toml` / workspace crate generation.

---

## [v0.7.0-modernization-2026] - 2026-09-15

This milestone delivers the **2026 Agentic Architecture Modernization**, implementing Model Context Protocol (MCP) 2026 conformance, hybrid test-time compute routing, step-level invariant telemetry, GRPO reward tracking, AST-netlist coupling in GraphRAG, and deep-thinking trace visualization in Oxide Agent Studio.

### Major Upgrades & Enhancements

#### 1. MCP 2026 Protocol Modernization & Direct Inference (`crates/mcp-server`)
- **Parameterized Scope**: Deprecated client `file://` Roots in favor of explicit `target_workspace_id` parameters and strict sandbox boundary enforcement.
- **Decoupled Reverse Sampling**: Embedded `Arc<dyn vllm_client::InferenceProvider>` directly into the MCP server for autonomous semantic tools (`analyze_compiler_failure`, `autonomous_code_review`).
- **Streamable HTTP & SSE**: Implemented `/api/agent/stream` Server-Sent Events (SSE) streaming endpoint supporting Multi Round-Trip Requests (MRTR).

#### 2. Hybrid Reasoning & Trace Soundness Verification (`workspace/router` & `crates/formal-verify`)
- **Dynamic `<think>` Compute Allocation**: Implemented `TaskComplexity` classifier (`Routine`, `Moderate`, `DeepReasoning`, `FormalProof`) allocating up to 16,384 thinking tokens.
- **Trace Soundness Judgement**: Created `TraceValidator` in `formal-verify` for extracting thought traces, evaluating backtracking/reflection, and detecting soundness violations (e.g. `.unwrap()` in embedded paths or `std` in `no_std`).

#### 3. Step-Level Invariant Metrics & Telemetry (`crates/benchmark-harness` & `crates/optio`)
- **Granular Evaluation Score**: Added `StepInvariantMetrics` tracking tool selection precision, JSON schema validity, recovery efficiency, and cost per success.
- **Real-Time Cost Tracking**: Added `estimated_cost_dollars()` to `BudgetTracker` for live GPU token cost accounting.

#### 4. GRPO Reinforcement Learning & Self-Evolution (`crates/self-evolver`)
- **Verification Reward Trajectories**: Enriched `VerificationDelta` with `reward_score`, `verification_engine`, and `domain` to harvest verified repair trajectories for group relative policy optimization.

#### 5. Temporal GraphRAG & Continuous Memory Consolidation (`crates/rag-pipeline` & `workspace/memory`)
- **AST-to-Netlist Coupling**: Added `query_ast_netlist_coupling` linking firmware symbols (e.g. GPIO/SPI registers) directly to schematic pins.
- **Background Memory Consolidation**: Implemented `consolidate_and_compress` in `WorkingMemoryManager` for episodic buffer compaction.

#### 6. Oxide Agent Studio UI/UX (`ui/oxide-agent-studio`)
- **Collapsible Reasoning Drawer**: Interactive `<think>` thought trace rendering with live token counters and complexity indicators.
- **Secondary Reviewer HITL Safety Badge**: LLM-as-judge confidence metrics and safety verdict indicators in `HitlApprovalModal.tsx`.

---

## [v0.6.0-oxide-protocol] - 2026-09-14

This milestone delivers **The Oxide Protocol**, establishing cross-domain atomic transactions, unified multi-domain context packing, and an automated multi-physics co-simulation loop spanning Firmware (IDE), Electronics (EDA), and Mechanics (3D CAD).

### Major Upgrades & Enhancements

#### 1. The Oxide-MCP Specification & Distributed Transactions (`crates/oxide-protocol`)
- **UUIDv7 DTX Generation**: Implemented `DtxId` using timestamp-ordered UUIDv7 for distributed transaction sequencing.
- **JSON-RPC 2.0 Schemas**: Standardized schemas for EDA (`eda.place_component`, `eda.route_differential_pair`, `eda.run_erc`) and 3D CAD (`cad.import_step`, `cad.brep_boolean`, `sim.run_fea_thermal`).
- **Distributed Coordinator (`workspace/scheduler/src/dtx_coordinator.rs`)**: Implemented `DtxCoordinator` managing multi-domain transaction state machines and atomic rollbacks (`rollback_dtx`).
- **MCP Client DTX Propagation (`workspace/mcp-clients/src/lib.rs`)**: Added `call_tool_with_dtx` enabling automatic DTX header propagation.

#### 2. Multi-Domain Context Packing (`workspace/memory/src/cross_domain_packer.rs`)
- **Cross-Domain Topology Slicing**: Implemented `CrossDomainContextPacker` uniting Firmware AST, PCB netlists, and CAD feature trees within strict token budgets.
- **Cross-Domain Graph Relationships**: Defined `code_symbol -> maps_to -> eda_component`, `eda_component -> mates_with -> cad_body`, and `code_function -> constrains -> cad_feature`.

#### 3. Autonomous Electro-Thermal-Mechanical Co-Simulation (`crates/cross-domain-verifier`)
- **Multi-Physics Verification Loop**: Coupled MCU firmware duty cycles with PCB dynamic power draw and CAD finite element thermal simulation.
- **Autonomous Remediation**: Automatically flags silicon junction temperatures exceeding $85^\circ\text{C}$ and triggers heatsink fin and thermal via generation.

#### 4. Unified Production CLI & Packaging (`src/main.rs` & `scripts/package_deb.sh`)
- **Unified `oxide-agent` CLI**: Implemented `daemon`, `doctor`, `re-forge`, `verify`, `status`, and `studio` subcommands.
- **Production Debian Packaging**: Generates standalone release `.deb` packages with systemd service and offline asset caching.

---

## [v0.5.0-re-forge-os] - 2026-09-14

This milestone establishes **Oxide-Tech Local Agent OS** as a trustworthy, local-first, graph-aware, sandboxed operating system for deterministic systems engineering, embedded hardware, open-source EDA, and high-performance binary/GPU reverse engineering.

### Major Upgrades & Enhancements

#### 1. Pure-Rust Binary Reverse Engineering (`crates/re-forge`)
- **Zero-Copy CPU Disassembly**: Integrated `goblin` format loaders (ELF/PE/Mach-O) and `yaxpeax-arch` x86_64/ARM instruction decoders.
- **Petgraph Control Flow Graph (CFG)**: Structured basic blocks, branch conditions, and function call boundaries into directed graphs.
- **Neural Decompilation to Safe Rust**: Automated extraction and prompt pipeline translating low-level assembly into idiomatic, safe, and typed Rust code.

#### 2. GPU Binary Reverse Engineering & cuDNN Lifting (`crates/re-forge/src/cuda`)
- **PTX & SASS Extraction**: Built `CudaAnalyzer` wrapping `cuobjdump` and `nvdisasm` inside the secure `bwrap` sandbox.
- **Tensor Core & Architecture Pattern Detection**: Implemented `PtxParser` identifying Ampere/Hopper Tensor Core instructions (`mma.sync`), shared memory tiling (`ld.shared`, `st.shared`), and asynchronous global copies (`cp.async`).
- **Neural CUDA C++ Lifter**: Reconstructs high-level algorithmic logic (Implicit GEMM, Winograd Convolution, FlashAttention) with `__half2` and `__nv_bfloat16` data layouts.

#### 3. Pluggable Inference Layer & Operating Profiles (`crates/config-loader` & `crates/vllm-client`)
- **InferenceProvider Abstraction**: Added unified async trait for Ollama, SGLang, vLLM, and Candle.
- **Operating Profiles**: Added first-class `--profile lite|standard|pro|airgapped|enterprise` configurations.
- **Ollama Provider**: Native HTTP client enabling full agent execution on CPU/low-VRAM devices with `qwen2.5-coder:7b`.

#### 4. Human-in-the-Loop (HITL) Interrupt Protocol (`workspace/scheduler/src/hitl.rs`)
- **Interactive Risk Gating**: Built `HitlApprovalChannel` using `tokio::sync::oneshot` channels, risk level classification, and decision routing to protect against unauthorized hardware flashes or destructive operations.

#### 5. Deterministic Evidence Bundles (`workspace/verifier/src/evidence.rs`)
- **Structured Audit Bundles**: Built `EvidenceBundle` generator exporting `task.json`, `patch.diff`, `verifier_reports.json`, and `hitl_decision.json`.

---

## [v0.4.0-agentic] - 2026-09-08

This release delivers comprehensive architectural improvements inspired by state-of-the-art agentic orchestration engines (LangGraph, CrewAI, AutoGen, Restate, OpenWorker):

### Major Upgrades & Enhancements

#### 1. Durable Event-Sourced Journaling (`crates/agent-journal`)
- **Event-Sourced Task Ledger**: Implemented `AgentJournal` & `JournalEvent` (`TaskSpawned`, `StepStarted`, `ToolCallDispatched`, `ToolCallFinished`, `HumanInterruptRequested`, `Completed`, `Failed`).
- **Crash Recovery Replay**: Added `ReplayedDagState::from_entries()` enabling deterministic state reconstruction and task continuation after node or daemon restarts.

#### 2. Distributed Observability & OpenTelemetry (`crates/telemetry`)
- **Distributed OTLP gRPC Tracing**: Added structured span export to Jaeger, Honeycomb, and OpenTelemetry collectors.
- **Trace Context Propagation & Attribution**: Instrumented LLM calls and tool dispatches with standard semantic conventions (`agent.id`, `dag.task_id`, `llm.model`, `llm.tokens`).

#### 3. Cyclic FSM Routing & Self-Correction (`workspace/router/src/agent_fsm.rs`)
- **Cyclic Agent FSM**: Implemented `AgentFsmRouter` with dynamic state transitions (`Pending` $\leftrightarrow$ `Executing` $\leftrightarrow$ `Diagnosing` $\leftrightarrow$ `Reviewing`).
- **Automated Diagnosis Escalation**: Configurable retry budgets automatically routing stubborn errors to dedicated diagnostic personas or human approval.

#### 4. Human-in-the-Loop (HITL) Inbox Protocol (`workspace/scheduler/src/inbox.rs`)
- **Suspension & Resume Channels**: Built `HitlInboxManager` with `tokio::sync::oneshot` channels to park execution on high-risk operations and resume on operator feedback.

#### 5. Parallel DAG Execution Engine (`workspace/router/src/supervisor.rs`)
- **Topological Tier Slicing**: Implemented `parallel_execution_tiers()` grouping independent task branches into concurrent execution waves.

#### 6. Scoped Working Memory (`workspace/memory/src/working_memory.rs`)
- **Multi-Tier Scopes**: Added `Global`, `Session`, `Task`, and `Scratchpad` scopes with token budgeting and automatic TTL eviction.

#### 7. Multi-Domain Dynamic LoRA Matching (`workspace/router/src/lora_router.rs`)
- **Domain Keyword Scoring**: Enhanced adapter classification across STM32, Redox OS, Tauri/Slint, WebAssembly, and systems programming domains.

#### 8. Operational Modes & Guardrails (`workspace/router/src/agent_modes.rs`)
- **Mode Enforcement**: Added `Plan`, `Code`, `Review`, `Debug`, `Architect`, and `Research` operational modes with strict write and execution controls.
