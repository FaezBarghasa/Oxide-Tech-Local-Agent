# Oxide-Tech Local Agent OS: Changelog

All notable changes to the Oxide-Tech Local Agent OS (NexusForge) codebase are documented here.

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
 
---
 
## [v0.3.0-nexusforge] - 2026-08-23

This major release completes the implementation of the **NexusForge 5-Layer Core Architecture**, featuring multi-modal graph engineering, semantic-structural hybrid indexing, dynamic LoRA hot-swapping, atomic checkpointer rollbacks, and tri-fold self-evolution.

### Major Upgrades & Enhancements

#### 1. Graph Engineering Layer (`workspace/knowledge` & `crates/optio`)
- **Tree-sitter AST & Call-Graph Parser**: Implemented `AstGraphExtractor` with typed `CodeGraphNode` and `CodeGraphEdge` extraction.
- **Topological Impact Analysis**: Implemented $K$-hop BFS blast radius calculation with downstream file mapping and targeted test selection.
- **Subgraph Context Pruner**: Implemented JSON context slicing yielding 75%–89% token prompt reductions.
- **Interactive UI**: Built `GraphTopologyTab.tsx` with live node exploration, blast radius filters, and token savings visualizers.

#### 2. Context & Memory Layer (`workspace/memory`)
- **Ephemeral Ring Buffers**: Implemented lock-free concurrent ring buffers for terminal session streams, editor dirty buffers, and stack traces.
- **Temporal Git History**: Implemented commit churn trajectory analysis and file-pair co-change coupling.

#### 3. Agent Engineering Layer (`workspace/router` & `crates/vllm-client`)
- **Multi-Persona Supervisor**: Created supervisor swarm orchestrating personas (`Lead Architect`, `Firmware Coder`, `EDA Engineer`, `DRC Reviewer`).
- **Dynamic LoRA Router**: Implemented heuristic task classification and runtime `/v1/lora/activate` dispatching to SGLang TP=2.

#### 4. Loop & Execution Layer (`crates/optio` & `workspace/verifier`)
- **Meta-Cognitive Observer**: Implemented token loop efficiency scoring and hallucination detection.
- **Oscillation Guard**: Added cyclic error detection preventing infinite retry loops.
- **Atomic Checkpointer**: Implemented `git stash create` snapshotting with instant rollbacks.

#### 5. Self-Evolution Engine (`crates/self-evolver`)
- **DeltaHarvester**: Implemented unified diff harvesting logging compiler fixes to `grpo_training_pool` in SurrealDB.
- **JIT MCP Tool Synthesizer**: Implemented standalone tool synthesis with Bubblewrap (`bwrap`) unshared namespace validation.
- **SkillOpt Crystallizer**: Created workflow distillation into structured `SKILL.md` documents.

#### 6. Inference Benchmarking (`scripts/`)
- **Ornith-1.0-9B GGUF Benchmark**: Executed live GPU inference and multi-LoRA tuning benchmark (`lora_embedded_rust_v2`, `lora_kicad_schgen_v3`, `delta_grpo_feedback`), achieving **8.59 tok/s average throughput** and 100% precision compiler self-healing.

---

## [v0.2.0-beta] - 2026-06-21
- SurrealDB v3 migration and schema models.
- Parallel HTTP/3 (QUIC / Quinn) and Actix-web dual gateway.
- Qdrant v1.18.0 hybrid vector indexing.
- Async worker protection and non-blocking actor isolation.

---

## [v0.1.0] - 2026-06-21
- Initial release of core platform, cargo check sandbox, and KiCad/SKiDL Python bridge.
