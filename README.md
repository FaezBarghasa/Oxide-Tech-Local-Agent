# Oxide-Tech Local Agent OS

**Oxide-Tech Local Agent OS** is a local-first, graph-aware, sandboxed agentic engineering operating system designed for deterministic software, embedded firmware, open-source EDA, high-performance binary/GPU reverse engineering, and multi-physics co-simulation workflows.

---

## 🎯 Strategic Positioning & Operating Profiles

Oxide-Tech provides 5 target operating modes adapting from lightweight laptop environments to multi-GPU enterprise air-gapped clusters:

| Operating Profile | Flag | Inference Backend | Default Model | Storage & Sandboxing |
| --- | --- | --- | --- | --- |
| **Lite Mode** | `--profile lite` | Local Ollama / `llama.cpp` | `gemma-4:9B` | Embedded SurrealKV / in-memory, read-only sandbox default |
| **Standard Mode** | `--profile standard` | Local Ollama / vLLM | `gemma-4-moe:26B` | Local SurrealDB + Qdrant, `bwrap` namespace sandbox |
| **Pro Mode** | `--profile pro` | SGLang / vLLM (TP=2) | `Ornith-1.5:35B` | SurrealDB + Qdrant, LoRA hot-swapping, full verifiers |
| **Air-Gapped Mode** | `--profile airgapped` | Pure offline weights | `qwen3.8:27b` | Zero WAN, offline doc index, signed tool manifests |
| **Enterprise Mode** | `--profile enterprise` | Local Cluster / Private API | Multi-model pipeline | Audit journal, RBAC, cryptographic evidence bundles |

---

## 🏗️ Architectural Topology & The Oxide Protocol

```
┌────────────────────────────────────────────────────────────────────────┐
│ User Interfaces (React 19 Studio, oxide-agent CLI, IDE Adapters)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ JSON-RPC 2.0 / SSE / QUIC HTTP/3
┌───────────────────────────────────▼────────────────────────────────────┐
│ Gateway & Control Plane (crates/gateway, :8080)                        │
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
| **`gateway`** | `crates/gateway/` | Dual-protocol high-performance gateway (Actix-web HTTP/2 + Quinn QUIC HTTP/3 + SSE streamable endpoints). |
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
| **`forge-rust`** | `crates/forge-rust/` | Polyglot-to-Rust refactoring engine (C/C++, Python, TypeScript, Go, Java, Generic) lifting to UIR, converting ownership/errors/concurrency to idiomatic Rust 2024 with `syn` validation and Cargo crate scaffolding. |
| **`cross-domain-verifier`** | `crates/cross-domain-verifier/` | Multi-physics electro-thermal-mechanical co-simulation loop (firmware duty cycle $\to$ PCB wattage $\to$ CAD thermal FEA mesh). |
| **`re-forge`** | `crates/re-forge/` | Zero-copy pure-Rust CPU binary disassembler (`goblin`, `yaxpeax-arch`, `petgraph`) & GPU/CUDA cuDNN lifting (PTX parser, Tensor Core detection, neural decompilation to safe Rust and CUDA C++). |
| **`circuit-forge`** | `crates/circuit-forge/` | EDA schematic builder, Electrical Rule Checking (ERC), topology analysis, and native KiCad S-expression serialization. |
| **`cad-forge`** | `crates/cad-forge/` | Parametric 3D CAD modeling, B-Rep geometric kernel, and voxelized clearance validation. |
| **`scene-forge`** | `crates/scene-forge/` | 3D scene graph, WGPU rendering pipeline integration, and Glam transformation matrices. |
| **`parametric-forge`** | `crates/parametric-forge/` | Constraint solver for parametric sketches and kinematic linkages. |
| **`visual-forge`** | `crates/visual-forge/` | Visual diffing, aesthetic inspection, and design system compliance checking. |
| **`web-forge`** | `crates/web-forge/` | Web front-end component generation and full-stack template synthesis. |
| **`wasm-forge`** | `crates/wasm-forge/` | Wasmtime WASI 0.2 sandboxed plugin execution host. |
| **`ratchet`** | `crates/ratchet/` | Dynamic native FFI hot-reloading and ABI validation runtime. |
| **`surface-api`** | `crates/surface-api/` | Direct GPU compute and surface memory transfer abstractions. |
| **`edge-swarm`** | `crates/edge-swarm/` | Decentralized edge agent gossip protocol and mesh coordination. |
| **`model-trainer`** | `crates/model-trainer/` | Local LoRA fine-tuning and GRPO reinforcement learning loops. |
| **`optio`** | `crates/optio/` | ReAct DAG orchestration engine, Personalized PageRank (PPR) AST slicing, oscillation guard, and task budgets. |
| **`sandbox`** | `crates/sandbox/` | Bubblewrap (`bwrap`) Linux namespace sandbox with resource caps and unshared PID/mount namespaces. |
| **`vllm-client`** | `crates/vllm-client/` | Pluggable `InferenceProvider` (Ollama, SGLang, vLLM, Candle) with LoRA adapter hot-swapping. |
| **`agent-journal`** | `crates/agent-journal/` | Event-sourced execution journaling using `rkyv` with deterministic state replay. |
| **`config-loader`** | `crates/config-loader/` | Profile-aware configuration manager (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`). |
| **`formal-verify`** | `crates/formal-verify/` | Bounded model checking, Kani formal proof generator, and LLM-as-Judge `TraceValidator` for soundness and hallucination checks. |
| **`benchmark-harness`** | `crates/benchmark-harness/` | Multi-suite agent evaluation harness (ARC-AGI, GAIA, SWE-bench) with `StepInvariantMetrics` (tool accuracy, schema validity, recovery, cost). |
| **`rag-pipeline`** | `crates/rag-pipeline/` | Hybrid retrieval with Tree-sitter AST symbol extraction and AST-to-netlist GraphRAG coupling. |
| **`qdrant-service`** | `crates/qdrant-service/` | Local Qdrant vector database client for dense AST embeddings and semantic code search. |
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
| **`ebpf-sentinel`** | `crates/ebpf-sentinel/` | Kernel-level LSM probe sandbox enforcing strict filesystem and hardware peripheral confinement. |
| **`self-evolver`** | `crates/self-evolver/` | GRPO reward harvesting (`VerificationDelta`), skill crystallization (`SKILL.md`), and automated tool synthesis. |

---

## 🖥️ Desktop-First Application & Unified CLI

Oxide-Tech Local Agent is built as a **Desktop-First Application** powered by Tauri v2 and React 19 (`ui/oxide-agent-studio`). All agent capabilities—from hardware diagnostics and binary decompilation to verification and memory audits—are available directly within the desktop UI without needing the command line.

### Launching Desktop Studio

```bash
# Launch the desktop studio (Tauri v2)
cargo tauri dev
```

### Unified Headless CLI

For headless environments or automation scripts, the root binary `Oxide-Tech-Local-Agent` delegates directly to the shared engine:

```bash
# 1. Run system diagnostics
oxide-agent doctor

# 2. Start the Daemon
oxide-agent daemon --profile standard --port 8080

# 3. Disassemble and reverse-engineer a binary / PTX file
oxide-agent re-forge path/to/binary --arch x86_64 --decompile

# 4. Run deterministic verifier and export signed evidence bundle
oxide-agent verify --workspace . --export-evidence ./target/evidence

# 5. Check running daemon status
oxide-agent status
```

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
