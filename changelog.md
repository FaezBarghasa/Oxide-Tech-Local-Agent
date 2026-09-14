# Oxide-Tech Local Agent OS: Changelog

All notable changes to the Oxide-Tech Local Agent OS codebase are documented here.

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
