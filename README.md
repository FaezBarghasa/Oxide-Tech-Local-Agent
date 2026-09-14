# Oxide-Tech Local Agent OS

**Oxide-Tech Local Agent OS** is a local-first, graph-aware, sandboxed agentic engineering operating system for deterministic software, embedded firmware, open-source EDA, and high-performance binary/GPU reverse engineering workflows.

---

## 🎯 Strategic Positioning & Operating Profiles

Oxide-Tech provides 5 target operating modes to adapt from ultra-lightweight laptop environments to multi-GPU enterprise air-gapped clusters:

| Operating Profile | Flag | Inference Backend | Default Model | Storage & Sandboxing |
|---|---|---|---|---|
| **Lite Mode** | `--profile lite` | Local Ollama / `llama.cpp` | `qwen2.5-coder:7b` | Embedded SurrealKV / in-memory, read-only sandbox default |
| **Standard Mode** | `--profile standard` | Local Ollama / vLLM | `qwen2.5-coder:14b` | Local SurrealDB + Qdrant, `bwrap` namespace sandbox |
| **Pro Mode** | `--profile pro` | SGLang / vLLM (TP=2) | `qwen2.5-coder:32b+` | SurrealDB + Qdrant, LoRA hot-swapping, full verifiers |
| **Air-Gapped Mode** | `--profile airgapped` | Pure offline weights | `qwen2.5-coder:32b` | Zero WAN, offline doc index, signed tool manifests |
| **Enterprise Mode** | `--profile enterprise` | Local Cluster / Private API | Multi-model pipeline | Audit journal, RBAC, cryptographic evidence bundles |

---

## 🏗️ Architectural Topology

```
┌────────────────────────────────────────────────────────────┐
│ User Interfaces (React 19 Studio, CLI / TUI, IDE Adapters) │
└──────────────────────────────┬─────────────────────────────┘
                               │ JSON-RPC 2.0 / SSE / QUIC HTTP/3
┌──────────────────────────────▼─────────────────────────────┐
│ Gateway & Control Plane (crates/api, workspace/gateway)   │
│ - Actix-Web + Quinn HTTP/3, Auth tokens, Task budgets     │
└──────────────────────────────┬─────────────────────────────┘
                               │
┌──────────────────────────────▼─────────────────────────────┐
│ Orchestration & State Machine (crates/optio, router)       │
│ - Predetermined DAG workflows, Oscillation guard, HITL FSM │
└───────┬──────────────┬──────────────┬──────────────┬───────┘
        │              │              │              │
┌───────▼──────┐ ┌─────▼─────┐ ┌──────▼──────┐ ┌────▼─────┐
│ Knowledge    │ │ Memory/RAG│ │ Verifier    │ │ Evolver  │
│ Graph AST    │ │ SurrealKV │ │ Sandbox     │ │ Skills   │
│ (SurrealDB)  │ │ + Qdrant  │ │ (bwrap/wasm)│ │ Manifests│
└───────┬──────┘ └─────┬─────┘ └──────┬──────┘ └────┬─────┘
        │              │              │              │
┌───────▼──────────────▼──────────────▼──────────────▼──────┐
│ Tool Execution & Domain Engineering Stack                  │
│ - re-forge: Pure-Rust Binary RE (CPU + CUDA/cuDNN PTX/SASS)│
│ - circuit-forge: Schematic EDA ERC/DRC & KiCad S-Expr      │
│ - mcp-probe-rs: Hardware-in-the-Loop RTT & STM32 flashing  │
│ - mcp-qemu-redox: QEMU simulation & kernel panic analysis  │
│ - mcp-cargo-gatekeeper: Dependency audit & policy checks   │
└────────────────────────────────────────────────────────────┘
        │              │              │              │
┌───────▼──────────────▼──────────────▼──────────────▼──────┐
│ Infrastructure & Storage                                   │
│ - SurrealDB / SurrealKV, Qdrant, agent-journal, telemetry │
└────────────────────────────────────────────────────────────┘
```

---

## 📦 Workspace Crates Map

| Crate | Directory | Purpose |
|---|---|---|
| **`re-forge`** | `crates/re-forge/` | Zero-copy pure-Rust CPU binary disassembler (`goblin`, `yaxpeax-arch`, `petgraph`) & GPU/CUDA cuDNN lifting (PTX parser, Tensor Core detection, neural decompilation to safe Rust and CUDA C++). |
| **`circuit-forge`** | `crates/circuit-forge/` | EDA schematic builder, Electrical Rule Checking (ERC), topology analysis, and native KiCad S-expression serialization. |
| **`cad-forge`** | `crates/cad-forge/` | Parametric 3D CAD modeling, B-Rep geometric kernel, and mesh generation. |
| **`optio`** | `crates/optio/` | ReAct DAG orchestration engine, Personalized PageRank (PPR) AST slicing, oscillation guard, and task budgets. |
| **`sandbox`** | `crates/sandbox/` | Bubblewrap (`bwrap`) Linux namespace sandbox with resource caps and Docker containers. |
| **`vllm-client`** | `crates/vllm-client/` | Pluggable `InferenceProvider` (Ollama, SGLang, vLLM, Candle) with LoRA adapter hot-swapping. |
| **`agent-journal`** | `crates/agent-journal/` | Event-sourced execution journaling using `rkyv` with deterministic state replay. |
| **`config-loader`** | `crates/config-loader/` | Profile-aware configuration manager (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`). |
| **`verifier`** | `workspace/verifier/` | Deterministic verification, atomic checkpoints (`git stash create`), and structured Evidence Bundle exports (`task.json`, `patch.diff`, `verifier_reports.json`). |
| **`scheduler`** | `workspace/scheduler/` | Human-in-the-Loop (HITL) interrupt channels (`tokio::sync::oneshot`), GPU resource governor, and background cron jobs. |
| **`knowledge`** | `workspace/knowledge/` | Multi-modal code graph AST extraction (Tree-sitter), impact analysis, and SurrealDB schema mapping. |
| **`memory`** | `workspace/memory/` | Dual-tier scoped working memory (`Global`, `Session`, `Task`, `Scratchpad`), causal action graphs, and ephemeral ring buffers. |
| **`router`** | `workspace/router/` | Fast/Slow cascading intent router, multi-persona supervisor swarm, and mode enforcement. |
| **`self-evolver`** | `crates/self-evolver/` | Skill crystallization (`SKILL.md` + `manifest.json`), compiler diff harvesting, and shadow sandbox verification. |
| **`gateway`** | `workspace/gateway/` | Dual-protocol high-performance gateway (Actix-web HTTP/2 + Quinn QUIC HTTP/3). |

---

## ⚡ Quickstart & Verification

### 1. Run Automated Diagnostics
```bash
./scripts/doctor.sh
```
Checks for Pop!_OS 24.04 toolchains, Rust 1.85+, Node.js, `pnpm`, `bwrap`, `probe-rs`, QEMU, and NVIDIA GPU acceleration.

### 2. Install Embedded Probe Rules (Optional for HW debug)
```bash
sudo ./scripts/install_udev_rules.sh
```

### 3. Build & Test Workspace
```bash
# Verify all workspace crates compile cleanly
cargo check --workspace

# Run full test suite
cargo test --workspace

# Test individual domain engines
cargo test -p re-forge
cargo test -p circuit-forge
cargo test -p verifier
```

### 4. Launch Gateway with Profile
```bash
# Lite Mode (CPU / Ollama)
cargo run -p gateway -- --profile lite

# Pro Mode (Multi-GPU / SGLang)
cargo run -p gateway -- --profile pro
```

---

## 📄 License

Licensed under the **Apache License, Version 2.0 with LLVM Exceptions** ([`LICENSE`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/LICENSE)).
