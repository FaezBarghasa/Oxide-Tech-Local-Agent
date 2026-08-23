# Oxide-Tech Local Agent OS

**Oxide-Tech Local Agent OS** (NexusForge) is the premier local-first, high-performance, self-evolving cognitive operating system designed for hardware and software engineering. It combines multi-modal code graph topology mapping, dynamic LoRA adapter hot-swapping, atomic sandbox execution, and tri-fold self-evolution.

---

## 1. Architectural Layers & System Topology

```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                       USER & IDE WORKSPACE LAYER                                               │
│             Oxide-Tech-IDE (Tauri v2 + Monaco + AST Viewer)  │  oxide-agent-studio (React 19 + Vite)            │
└───────────────────────────────────────────────────────┬────────────────────────────────────────────────────────┘
                                                        │ JSON-RPC 2.0 / gRPC / SSE / QUIC HTTP/3
┌───────────────────────────────────────────────────────▼────────────────────────────────────────────────────────┐
│                          NEXUSFORGE / OXIDE-TECH-LOCAL-AGENT CORE ENGINE                                       │
│                                                                                                                │
│   ┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐   │
│   │                                      GRAPH ENGINEERING LAYER                                           │   │
│   │  ┌─────────────────────────┐  ┌──────────────────────────────┐  ┌───────────────────────────────────┐  │   │
│   │  │  Multi-Modal Code Graph │  │  Semantic-Structural Hybrid  │  │   Impact Analysis & Propagation   │  │   │
│   │  │  (AST, Call, Flow, Net) │  │  Indexing (Qdrant+SurrealQL) │  │   (Sub-graph AST Slicing -80%)    │  │   │
│   │  └────────────┬────────────┘  └──────────────┬───────────────┘  └─────────────────┬─────────────────┘  │   │
│   └───────────────┼──────────────────────────────┼────────────────────────────────────┼────────────────────┘   │
│                   │                              │                                    │                        │
│   ┌───────────────▼──────────────────────────────▼────────────────────────────────────▼────────────────────┐   │
│   │                                     CONTEXT & MEMORY LAYER                                             │   │
│   │  ┌─────────────────────────┐  ┌──────────────────────────────┐  ┌───────────────────────────────────┐  │   │
│   │  │ Temporal History (Git)  │  │ Dual-State Memory Engine     │  │ Ephemeral Ring Buffers            │  │   │
│   │  │ Graph-Node Versioning   │  │ (SurrealDB + Qdrant Embeds)  │  │ (Terminal, Editor, Stack Traces)  │  │   │
│   │  └─────────────────────────┘  └──────────────────────────────┘  └───────────────────────────────────┘  │   │
│   └──────────────────────────────────────────────┬─────────────────────────────────────────────────────────┘   │
│                                                  │                                                             │
│   ┌──────────────────────────────────────────────▼─────────────────────────────────────────────────────────┐   │
│   │                                       LOOP & EXECUTION LAYER                                           │   │
│   │  ┌─────────────────────────┐  ┌──────────────────────────────┐  ┌───────────────────────────────────┐  │   │
│   │  │ Optio ReAct DAG Engine  │  │ Atomic Checkpointer & Rollback│ │ Deterministic Verifiers           │  │   │
│   │  │ (Oscillation Guard)     │  │ (bwrap / git stash create)   │  │ (cargo check, kicad-cli, QEMU)    │  │   │
│   │  └─────────────────────────┘  └──────────────────────────────┘  └───────────────────────────────────┘  │   │
│   └──────────────────────────────────────────────┬─────────────────────────────────────────────────────────┘   │
│                                                  │                                                             │
│   ┌──────────────────────────────────────────────▼─────────────────────────────────────────────────────────┐   │
│   │                                      SELF-EVOLUTION ENGINE                                             │   │
│   │  ┌─────────────────────────┐  ┌──────────────────────────────┐  ┌───────────────────────────────────┐  │   │
│   │  │ JIT MCP Tool Synthesizer│  │ SkillOpt Workflow Curator    │  │ Delta-RL Harvester & Trainer      │  │   │
│   │  │ (Mojo SIMD / Sandbox)   │  │ (Crystallized Skill Graph)   │  │ (SurrealDB grpo_training_pool)    │  │   │
│   │  └─────────────────────────┘  └──────────────────────────────┘  └───────────────────────────────────┘  │   │
│   └────────────────────────────────────────────────────────────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────┬────────────────────────────────────────────────────────┘
                                                        │
┌───────────────────────────────────────────────────────▼────────────────────────────────────────────────────────┐
│                        DUAL NVIDIA RTX ACCELERATION & INFERENCE ENGINE                                         │
│   ┌─────────────────────────────────────────────────┐   ┌──────────────────────────────────────────────────┐   │
│   │         GPU 0 (PCIe 4.0 x16 - 24GB VRAM)        │   │         GPU 1 (PCIe 4.0 x16 - 24GB VRAM)         │   │
│   │ - SGLang Tensor Parallel Rank 0 (Qwen3.8-35B)   │◄──┼►- SGLang Tensor Parallel Rank 1 (Qwen3.8-35B)    │   │
│   │ - Fast Intent Router (Gemma-4-9B / Ornith-1.0)  │NCC│ - SF3D / B-Rep Latent Geometry Diffusion Engine  │   │
│   │ - Mojo SIMD Vector & Netlist Acceleration Core  │ L │ - Dynamic LoRA Adapter Cache (Rust/KiCad/CAD)    │   │
│   └─────────────────────────────────────────────────┘   └──────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Workspace Crates & Architecture

| Crate / Module | Path | Description |
|---|---|---|
| `knowledge` | `workspace/knowledge/` | Multi-modal code graph, Tree-sitter AST extraction (`AstGraphExtractor`), impact blast radius, and subgraph context pruning. |
| `memory` | `workspace/memory/` | Ephemeral ring buffers (editor, terminal logs, stack traces) and temporal git churn / co-change tracking. |
| `router` | `workspace/router/` | Multi-persona supervisor swarm, meta-cognitive observer agent, and dynamic LoRA adapter hot-swapping. |
| `optio` | `crates/optio/` | ReAct DAG engine, oscillation guard, impact analysis, context slicing, and persona orchestrator. |
| `verifier` | `workspace/verifier/` | Atomic checkpoint snapshotting (`git stash create`), rollback engine, and deterministic firmware validation. |
| `self-evolver` | `crates/self-evolver/` | Delta-based reinforcement harvester (`grpo_training_pool`), JIT MCP tool synthesizer (`bwrap`), and workflow skill crystallization. |
| `surrealdb-service` | `crates/surrealdb-service/` | SurrealDB v3 schema definitions (`code_node`, `calls`, `defines`, `implements`, `references`, `data_flows_to`). |
| `qdrant-service` | `crates/qdrant-service/` | High-dimensional semantic-structural hybrid vector indexer. |
| `vllm-client` | `crates/vllm-client/` | SGLang TP=2 runtime client and `/v1/lora/activate` hot-swap dispatcher. |
| `gateway` | `workspace/gateway/` | High-performance dual-protocol gateway (Actix-web TCP + Quinn QUIC HTTP/3 on port `8080`). |
| `oxide-agent-studio` | `ui/oxide-agent-studio/` | React 19 + Vite web studio with interactive Graph Engineering, Agent Loops, Verifiers, and LoRA controllers. |

---

## 3. Quickstart & Testing

### Prerequisites
- **Rust**: 1.85+ (Edition 2021 / 2024)
- **Node.js & pnpm**: 20+ (`pnpm` required for UI)
- **Python / uv**: Python 3.11+ and `uv`
- **SurrealDB & Qdrant**: SurrealDB 3.x and Qdrant 1.18.x

### Build & Run Tests
```bash
# Verify all 27 workspace crates
cargo check --workspace

# Run full automated test suite across all subsystems
cargo test --workspace

# Test AST parsing and code graph
cargo test -p knowledge --lib ast::tests
cargo test -p knowledge --test code_graph_test

# Test Supervisor and Dynamic LoRA Router
cargo test -p router --test supervisor_test
cargo test -p router --test agent_loop_test
cargo test -p router --test ornith_test

# Test Self-Evolution & Delta Harvester
cargo test -p self-evolver
```

### Launch Local Stack & UI Studio
```bash
# 1. Launch backend services & SGLang serving stack
./scripts/launch_stack.sh

# 2. Launch Oxide Agent Studio UI (built with pnpm)
cd ui/oxide-agent-studio
pnpm install
pnpm build
pnpm start
# Open http://localhost:3000 in your browser
```

### Run Ornith-1.0-9B Inference & Tuning Benchmark
```bash
uv run --with llama-cpp-python python3 scripts/run_and_tune_benchmark.py
```
