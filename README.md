# Oxide-Tech Local Agent OS

**Oxide-Tech Local Agent OS** is a local-first, graph-aware, sandboxed agentic engineering operating system designed for deterministic software, embedded firmware, open-source EDA, high-performance binary/GPU reverse engineering, multi-physics co-simulation, and universal multi-provider AI model orchestration.

---

## 🎯 Strategic Positioning & Operating Profiles

Oxide-Tech provides 5 target operating modes adapting from lightweight laptop environments to multi-GPU enterprise air-gapped clusters:

| Operating Profile | Flag | Inference Backend | Default Model | Storage & Sandboxing |
| --- | --- | --- | --- | --- |
| **Lite Mode** | `--profile lite` | Local `llama-server` / Ollama | `gemma4-v2-Q3_K_M.gguf` | Embedded SurrealKV / in-memory, read-only sandbox default |
| **Standard Mode** | `--profile standard` | Local `llama-server` / vLLM | `Ternary-Bonsai-2-27B-Abliterated` | Local SurrealDB + Qdrant, `bwrap` namespace sandbox |
| **Pro Mode** | `--profile pro` | SGLang / vLLM (TP=2) | `DeepSeek-R1-0528-Qwen3-8B` | SurrealDB + Qdrant, LoRA hot-swapping, full verifiers |
| **Air-Gapped Mode** | `--profile airgapped` | Pure offline `.gguf` weights | `Ornith-1.5-9B-Q4_K_M.gguf` | Zero WAN, offline doc index, signed tool manifests |
| **Enterprise Mode** | `--profile enterprise` | Universal Gateway / Private API | Multi-model pipeline | Audit journal, RBAC, cryptographic evidence bundles |

---

## 🌐 Universal AI Gateway & Native Engine Matrix (v3.2)

Oxide-Tech incorporates a pure-Rust **Universal AI Gateway** and zero-stub **Native Inference Orchestrator**, providing parity with LM Studio and Unsloth Studio:

### 1. 19 Universal Routing Strategies (`crates/oxide-gateway/src/universal_router.rs`)
- **Traffic Balancing**: `round-robin`, `weighted`, `random`, `strict-random`, `p2c` (power-of-two-choices), `least-used`, `chaos` (chaos-engineering fault injection).
- **Latency & Capacity Gating**: `priority`, `fill-first`, `headroom`, `cost-optimized`, `lkgp` (last-known-good-provider).
- **Rate-Limit & Reset Awareness**: `reset-window` (sliding token bucket), `reset-aware` (RPM/RPD decay tracker).
- **Context Routing**: `context-relay` (automatic handoff when session tokens reach $\ge 85\%$ context limit), `context-optimized`, `cache-optimized` (KV cache affinity).
- **Heuristic Ensembles**: `auto` (16-factor dynamic multi-objective scoring), `fusion` (multi-candidate speculative ranking).

### 2. Zero-Stub Native Inference Runtime
- **On-Demand `llama-server` Daemon**: Managed GPU offloading (`-ngl 99`, `--ctx-size 8192`, `--port 8081`) with real-time health polling and PID lifecycle supervision.
- **Recursive Disk Discovery**: Auto-indexes local `.gguf` weights across `~/models`, `~/.cache/huggingface`, `~/.ollama/models`, and `/opt/models`.
- **Zero Fake Stubs**: Elimination of all mock/fallback synthesizers; every inference request executes genuine weights locally or routes via active cloud providers.

---

## 🏗️ Architectural Topology & The Oxide Protocol

```
┌────────────────────────────────────────────────────────────────────────┐
│ User Interfaces (React 19 Studio, oxide-agent CLI, IDE Adapters)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ JSON-RPC 2.0 / SSE / QUIC HTTP/3
┌───────────────────────────────────▼────────────────────────────────────┐
│ Gateway & Universal Control Plane (crates/oxide-gateway, :8080)        │
│ - 19 Routing Strategies + 16-Factor Scoring Engine                     │
│ - Native llama-server runtime (:8081) + Ollama bridge (:11434)        │
│ - Actix-Web + Quinn HTTP/3, JWT Guards, Prometheus /metrics            │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ The Oxide Protocol & Distributed Transactions (UUIDv7 DTX)             │
│ - DtxCoordinator (Atomic Multi-Domain Commits & rollback_dtx)          │
└───────┬──────────────┬──────────────┬──────────────┬───────────┬───────┘
        │              │              │              │           │
┌───────▼──────┐ ┌─────▼─────┐ ┌──────▼──────┐ ┌────▼─────┐ ┌────▼──────┐
│ Knowledge    │ │ Memory/RAG│ │ Verifier    │ │ Evolver  │ │ Hardware  │
│ Graph AST    │ │ SurrealKV │ │ Multi-      │ │ Skills   │ │ Simulator │
│ (SurrealDB)  │ │ + Qdrant  │ │ Physics     │ │ Manifests│ │ probe-rs  │
│              │ │ Context   │ │ FEA/ERC     │ │          │ │ QEMU Redox│
└───────┬──────┘ └─────┬─────┘ └──────┬──────┘ └────┬─────┘ └────┬──────┘
        │              │              │              │           │
┌───────▼──────────────▼──────────────▼──────────────▼───────────▼──────┐
│ Tool Execution & Domain Engineering Stack (crates/*)                  │
│ - re-forge: Pure-Rust Binary RE (CPU + CUDA/cuDNN PTX/SASS)           │
│ - forge-rust: Universal Polyglot to Idiomatic Rust 2024 Refactorer    │
│ - circuit-forge: Schematic EDA ERC/DRC & KiCad S-Expr                 │
│ - cad-forge: Parametric 3D CAD modeling & B-Rep kernel                │
│ - cross-domain-verifier: Electro-Thermal-Mechanical Co-Simulation     │
│ - mcp-probe-rs: Hardware-in-the-Loop RTT & STM32 flashing             │
│ - mcp-qemu-redox: QEMU simulation & kernel panic analysis             │
│ - mcp-cargo-gatekeeper: Dependency audit & policy checks              │
└───────────────────────────────────────────────────────────────────────┘
        │              │              │              │           │
┌───────▼──────────────▼──────────────▼──────────────▼───────────▼──────┐
│ Infrastructure & Security Membrane                                    │
│ - Bubblewrap (bwrap) Namespace Sandbox + ebpf-sentinel Syscall Guard  │
│ - SurrealDB / SurrealKV, Qdrant, agent-journal, OpenTelemetry Tracing │
└───────────────────────────────────────────────────────────────────────┘
```

---

## 📦 Workspace Crates Map (Consolidated `crates/`)

| Crate | Directory | Purpose & Agent Capability |
| --- | --- | --- |
| **`oxide-protocol`** | `crates/oxide-protocol/` | Universal communication specification, JSON-RPC 2.0 schemas for EDA/CAD, and time-ordered UUIDv7 Distributed Transaction IDs (`DtxId`). |
| **`oxide-gateway`** | `crates/oxide-gateway/` | Universal AI Gateway server with 19 routing strategies, 16-factor scoring, token bucket rate-limiting, and OpenAI-compatible proxying. |
| **`thinker`** | `crates/thinker/` | Autonomous reasoning loop, ReAct decision cycle, tool calling synthesizer, and multi-step plan decomposition. |
| **`memory`** | `crates/memory/` | CrossDomainContextPacker, scoped working memory (`Global`, `Session`, `Task`, `Scratchpad`), causal action graphs, and Memanto decision conflict auditor. |
| **`router`** | `crates/router/` | Fast/Slow cascading intent router, multi-persona supervisor swarm, and operational mode enforcement. |
| **`verifier`** | `crates/verifier/` | Deterministic verification, atomic checkpoints (`git stash create`), and structured Evidence Bundle exports. |
| **`knowledge`** | `crates/knowledge/` | Multi-modal code graph AST extraction (Tree-sitter), STAIR Code-ToC leaf search, impact analysis, and SurrealDB schema mapping. |
| **`skills`** | `crates/skills/` | Dynamic skill loading, YAML-frontmatter `SKILL.md` parser, and procedural execution harnesses. |
| **`mcp-clients`** | `crates/mcp-clients/` | Async client wrappers for external Model Context Protocol (MCP) servers and tool discovery. |
| **`common`** | `crates/common/` | Shared error types, serialization DTOs, logging macros, and core telemetry primitives. |
| **`mcp-server`** | `crates/mcp-server/` | Modern RMCP server with parameterized workspace boundaries and autonomous reasoning tools. |
| **`blog`** | `crates/blog/` | Automated technical documentation, mdBook knowledge base synchronization, and docstring crystallization. |
| **`scheduler`** | `crates/scheduler/` | Distributed Transaction Coordinator (`DtxCoordinator`), Human-in-the-Loop (HITL) interrupt channels, and GPU governor. |
| **`forge-rust`** | `crates/forge-rust/` | Polyglot-to-Rust refactoring engine lifting foreign code to UIR and synthesizing idiomatic Rust 2024. |
| **`cross-domain-verifier`** | `crates/cross-domain-verifier/` | Multi-physics electro-thermal-mechanical co-simulation loop. |
| **`re-forge`** | `crates/re-forge/` | Zero-copy pure-Rust CPU binary disassembler & GPU/CUDA cuDNN decompilation. |
| **`circuit-forge`** | `crates/circuit-forge/` | EDA schematic builder, Electrical Rule Checking (ERC), topology analysis, and KiCad S-expression serialization. |
| **`cad-forge`** | `crates/cad-forge/` | Parametric 3D CAD modeling, B-Rep geometric kernel, and voxelized clearance validation. |
| **`scene-forge`** | `crates/scene-forge/` | 3D scene graph, WGPU rendering pipeline integration, and Glam transformation matrices. |
| **`parametric-forge`** | `crates/parametric-forge/` | Constraint solver for parametric sketches and kinematic linkages. |
| **`visual-forge`** | `crates/visual-forge/` | Visual diffing, aesthetic inspection, and design system compliance checking. |
| **`web-forge`** | `crates/web-forge/` | Web front-end component generation and full-stack template synthesis. |
| **`wasm-forge`** | `crates/wasm-forge/` | Wasmtime WASI 0.2 sandboxed plugin execution host. |
| **`ratchet`** | `crates/ratchet/` | Dynamic native FFI hot-reloading and ABI validation runtime. |
| **`surface-api`** | `crates/surface-api/` | Direct GPU compute and surface memory transfer abstractions. |
| **`edge-swarm`** | `crates/edge-swarm/` | Decentralized edge agent gossip protocol and mesh coordination. |
| **`oxide-engines`** | `crates/oxide-engines/` | Pure-Rust non-autoregressive decision engine (`DecisionEngine`), dynamic micro-batching via Flume MPMC, `CandidateVectorCache`, Brier score calibration, and Fast-KAN head. |
| **`oxide-security`** | `crates/oxide-security/` | Ephemeral TLS bootstrap, `< 2GB` circuit-breaker resource gater, idempotent session receipts, and process tree containment (`setpgid`). |
| **`oxide-kernels`** | `crates/oxide-kernels/` | GPU architecture autotuning (Ampere, Ada, Hopper, Blackwell), tile sizing, and AVX-512F SIMD tensor compression. |
| **`oxide-state`** | `crates/oxide-state/` | Centralized agent state holding SurrealDB connection, `ResourceGater`, and model registries. |
| **`model-trainer`** | `crates/model-trainer/` | Safetensors sharding, Online RL (DPO, ORPO, GRPO), Multi-Node ZeRO-3 parameter sharding, and DDR5 RAM tier offloading. |
| **`optio`** | `crates/optio/` | ReAct DAG orchestration engine, Personalized PageRank (PPR) AST slicing, oscillation guard, and task budgets. |
| **`sandbox`** | `crates/sandbox/` | Bubblewrap (`bwrap`) Linux namespace sandbox with resource caps. |
| **`vllm-client`** | `crates/vllm-client/` | Pluggable `InferenceProvider` (Ollama, SGLang, vLLM, Candle) with LoRA adapter hot-swapping. |
| **`agent-journal`** | `crates/agent-journal/` | Event-sourced execution journaling using `rkyv` with deterministic state replay. |
| **`config-loader`** | `crates/config-loader/` | Profile-aware configuration manager (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`). |
| **`formal-verify`** | `crates/formal-verify/` | Bounded model checking, Kani formal proof generator, and LLM-as-Judge `TraceValidator`. |
| **`benchmark-harness`** | `crates/benchmark-harness/` | Multi-suite agent evaluation harness with `StepInvariantMetrics`. |
| **`rag-pipeline`** | `crates/rag-pipeline/` | Hybrid retrieval with Tree-sitter AST symbol extraction and GraphRAG. |
| **`qdrant-service`** | `crates/qdrant-service/` | Local Qdrant vector database client for dense AST embeddings. |
| **`surrealdb-service`** | `crates/surrealdb-service/` | Multi-model graph and document database engine for AST and memory storage. |
| **`tree-sitter-service`** | `crates/tree-sitter-service/` | Multi-language syntax tree parsing, symbol indexing, and AST extraction. |
| **`gateway-router`** | `crates/gateway-router/` | Fast routing tables and middleware filters for API gateways. |
| **`api`** | `crates/api/` | Actix-web and gRPC service definitions for edge agent communication. |
| **`telemetry`** | `crates/telemetry/` | Structured OpenTelemetry tracing, metrics collection, and Prometheus exporting. |
| **`crucible`** | `crates/crucible/` | Automated test generation, mutation testing, and adversarial fuzzing harness. |
| **`mcp-probe-rs`** | `crates/mcp-probe-rs/` | Hardware-in-the-Loop debugging, RTT streaming, and safe STM32/ARM flashing. |
| **`mcp-qemu-redox`** | `crates/mcp-qemu-redox/` | Headless microVM Redox OS emulation and kernel driver validation. |
| **`mcp-cargo-gatekeeper`** | `crates/mcp-cargo-gatekeeper/` | Sandboxed compiler checks, dependency security scanning, and policy gates. |
| **`mcp-live-docs`** | `crates/mcp-live-docs/` | Real-time offline datasheet search and technical documentation RAG. |
| **`oxide-tooling`** | `crates/oxide-tooling/` | In-process native agent tool calling (`NativeToolRegistry`), Ornith XML prompt formatter, and zero-latency tool dispatch (`StairSearchTool`, `MemoryRecallTool`, `HardwareProbeTool`, `PtxDecompileTool`). |
| **`oxide-network`** | `crates/oxide-network/` | Sovereign QUIC mesh transport, anti-DPI preamble filtering, 128-packet anti-replay window, and 16-byte encrypted wire frames. |
| **`oxide-core`** | `crates/oxide-core/` | Fundamental primitives, unified diff patcher, token streaming channels, FFI panic boundaries, and sealed shared memory. |
| **`audio-forge`** | `crates/audio-forge/` | Local audio transcription (Whisper STT), voice synthesis, and audio device streaming. |
| **`media-forge`** | `crates/media-forge/` | Local diffusion pipelines, image generation, and multi-modal asset synthesis. |
| **`ebpf-sentinel`** | `crates/ebpf-sentinel/` | Kernel-level LSM probe sandbox enforcing strict filesystem confinement. |
| **`self-evolver`** | `crates/self-evolver/` | GRPO reward harvesting (`VerificationDelta`), skill crystallization (`SKILL.md`), and automated tool synthesis. |

---

## 🏛️ Capability Horizons (Horizons 0 – VII) & Verification

The architecture is formally validated across 8 capability horizons via [`tests/pure_rust_stack_test.rs`](tests/pure_rust_stack_test.rs):

- **Horizon 0: Pure Rust Foundations**: GGUF mmap zero-copy loading, ternary GEMM (-1, 0, 1), embedded SurrealKV storage, and local Qdrant vectors.
- **Horizon I: Monolith Integration**: Single production executable (`oxide-tech-local-agent`) containing desktop UI, embedded gateway, and diagnostic engines with zero external Python runtimes.
- **Horizon II: In-Process Agent Tool Calling**: Native dispatch via [`NativeToolRegistry`](crates/oxide-tooling/src/native_tools.rs) with sub-10ms MoE expert routing SLA.
- **Horizon III: Closed-Loop EDA & Circuit Safety**: Electrical Rule Checking ([`circuit-forge`](crates/circuit-forge/)) decoupling analysis and SMT-LIB2 invariant proof generation ([`formal-verify`](crates/formal-verify/)).
- **Horizon IV: Mechanical CAD & B-Rep**: Constructive Solid Geometry (CSG) boolean difference and 3D [`VoxelGrid`](crates/cad-forge/src/voxel.rs) volumetric collision checks.
- **Horizon V: Sovereign QUIC Mesh Transport**: Wire protocol anti-replay filtering ([`ReplayWindow128`](crates/oxide-network/src/wire.rs)), anti-DPI junk preamble suppression, and 16-byte encrypted headers.
- **Horizon VI: Self-Evolution & Sandboxed WASI**: Dynamic Wasm tool execution ([`self-evolver`](crates/self-evolver/)) and automated skill crystallization to `SKILL.md`.
- **Horizon VII: Heterogeneous Cross-Domain Co-Simulation**: Electro-thermal-mechanical fixed-point relaxation loop ([`cross-domain-verifier`](crates/cross-domain-verifier/)) simulating MCU power dissipation against heat sink thermal resistance.


---

## 🖥️ Desktop-First Application & Unified Monolith CLI

Oxide-Tech Local Agent is engineered as a **Hermetic Single-Binary Desktop Monolith** powered by Tauri v2 and React 19 (`src/`). All agent capabilities—from in-process gateway server and hardware diagnostics to binary decompilation, verification suites, and STAIR Code-ToC project memory—are compiled directly into one production executable (`oxide-tech-local-agent`) with zero external service or Python runtime dependencies.

### Launching Desktop Studio

```bash
# Launch the desktop studio (Builds and runs the single monolith binary)
cargo run
# Or via Tauri dev runner
cargo tauri dev
```

### Unified Headless & Systems CLI

The single production binary `oxide-tech-local-agent` supports complete desktop, daemon, and diagnostic subcommands:

```bash
# 1. Launch Desktop UI (Embedded In-Process Gateway + STAIR Memory)
oxide-tech-local-agent desktop [--config PATH]

# 2. Self-Install Desktop Entry, udev rules & user paths
oxide-tech-local-agent --install

# 3. Start the Headless Gateway (Systemd Service Mode)
oxide-tech-local-agent daemon [--config PATH]

# 4. Run system diagnostics & toolchain checks
oxide-tech-local-agent doctor [--json]

# 5. Disassemble and reverse-engineer a binary / PTX file
oxide-tech-local-agent re-forge path/to/binary [--arch ARCH] [--json]

# 6. Run deterministic verifier and export evidence bundle
oxide-tech-local-agent verify [--workspace PATH] [--json]

# 7. STAIR Code-ToC & Memanto memory operations (in-process or CLI passthrough)
oxide-tech-local-agent memory <subcommand> [args...]

# 8. Probe running gateway liveness
oxide-tech-local-agent status
```

### In-Process Domain Controllers (`src-tauri/src/controllers/`)

The desktop IPC layer is organized using the **Controller Facade Pattern**, consolidating 18 discrete IPC handler modules into 4 domain controllers:
- **`AgentController`** ([`src-tauri/src/controllers/agent_controller.rs`](src-tauri/src/controllers/agent_controller.rs)): In-process ReAct reasoning loop, native tool dispatch (`NativeToolRegistry`), and prompt generation.
- **`SystemController`** ([`src-tauri/src/controllers/system_controller.rs`](src-tauri/src/controllers/system_controller.rs)): Hardware health diagnostics, probe-rs udev rules deployment, and dynamic configuration management.
- **`WorkspaceController`** ([`src-tauri/src/controllers/workspace_controller.rs`](src-tauri/src/controllers/workspace_controller.rs)): STAIR Code-ToC leaf search, Memanto decision conflict auditor, and semantic memory graph.
- **`ForgeController`** ([`src-tauri/src/controllers/forge_controller.rs`](src-tauri/src/controllers/forge_controller.rs)): Zero-copy binary reverse engineering, PTX decompilation, and deterministic verification suites.


---

## 📦 Packaging & Offline Deployment

- **Debian Package Build**:

  ```bash
  ./scripts/package_deb.sh
  ```

  Generates `target/debian/oxide-tech-local-agent_0.1.0_amd64.deb` containing the release binary, systemd service, and Studio desktop assets.
- **Offline Asset Cache**:

  ```bash
  ./scripts/bundle_offline.sh
  ```

  Caches ONNX embedding models, tree-sitter grammars, and documentation indices in `~/.cache/oxide-tech/` for zero-WAN / air-gapped execution.
