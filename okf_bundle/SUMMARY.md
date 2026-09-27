---
description: 'Top-level OKF summary: 7354 concepts across 8 domains and 433 modules'
git_branch: master
git_repo: Oxide-Tech-Local-Agent
okf_version: '0.2'
timestamp: '2026-09-27T20:18:57Z'
title: Oxide-Tech-Local-Agent — Knowledge Summary
type: Index
---

# Oxide-Tech-Local-Agent — Knowledge Summary

> OKF v0.2 bundle | 7,354 concepts | 8 domains | 433 modules

## Stats

| Type | Count |
|------|-------|
| Function | 3,590 |
| Dependency | 2,244 |
| Class | 952 |
| Module | 433 |
| Interface | 108 |
| Type | 26 |
| Resource | 1 |

| Language | Concepts |
|----------|----------|
| rust | 4,546 |
| manifest | 2,244 |
| typescript | 410 |
| python | 152 |
| yaml | 2 |

## Domain Map

Use these links to navigate the bundle or prime an AI agent with focused context.

### [crates](crates/index.md) — 4,264 concepts

- [crates/vllm-client/src/multi_agent](crates/vllm-client/src/multi_agent/index.md) (94 concepts)
- [crates/oxide-network/src/types](crates/oxide-network/src/types/index.md) (78 concepts) — # Core Types for Zero-Trust Remote Mesh Network (`crates/oxide-network/src/types
- [crates/oxide-mcp/src/handler](crates/oxide-mcp/src/handler/index.md) (64 concepts)
- [crates/oxide-network/src/crypto](crates/oxide-network/src/crypto/index.md) (62 concepts) — # Cryptographic Primitives for Zero-Trust Mesh Network (`crates/oxide-network/sr
- [crates/model-trainer/src/lib](crates/model-trainer/src/lib/index.md) (57 concepts) — # Model Trainer
- [crates/oxide-network/src/transport](crates/oxide-network/src/transport/index.md) (56 concepts) — # QUIC Datagram & Stream Transport Engine (`crates/oxide-network/src/transport.r
- [crates/oxide-network/src/wire](crates/oxide-network/src/wire/index.md) (49 concepts) — # Wire Protocol Framing for Data & Control Plane Packets (`crates/oxide-network/
- [crates/oxide-engines/src/decision_engine](crates/oxide-engines/src/decision_engine/index.md) (44 concepts)
- *…and 327 more modules*

### [generated](generated/index.md) — 210 concepts

- [generated/pcb_layout_generated](generated/pcb_layout_generated/index.md) (134 concepts)
- [generated/Oxide/FB/Component](generated/Oxide/FB/Component/index.md) (27 concepts)
- [generated/Oxide/FB/Layout](generated/Oxide/FB/Layout/index.md) (21 concepts)
- [generated/Oxide/FB/Pin](generated/Oxide/FB/Pin/index.md) (18 concepts)
- [generated/Oxide/FB/Vec3D](generated/Oxide/FB/Vec3D/index.md) (8 concepts)
- [generated/Oxide/FB/__init__](generated/Oxide/FB/__init__/index.md) (1 concepts)
- [generated/Oxide/__init__](generated/Oxide/__init__/index.md) (1 concepts)

### [python-bridge](python-bridge/index.md) — 60 concepts

- [python-bridge/bridge_pb2_grpc](python-bridge/bridge_pb2_grpc/index.md) (10 concepts) — Client and server classes corresponding to protobuf-defined services.
- [python-bridge/server](python-bridge/server/index.md) (10 concepts)
- [python-bridge/scrapling_bridge](python-bridge/scrapling_bridge/index.md) (9 concepts) — Oxide-Tech Local Agent: Scrapling Stealth Web Perception Worker
- [python-bridge/training/train_thermal_model](python-bridge/training/train_thermal_model/index.md) (6 concepts) — Oxide-Tech Circuit Forge: Graph Neural Network (GNN) Thermal Solver
- [python-bridge/kicad_bridge](python-bridge/kicad_bridge/index.md) (4 concepts)
- [python-bridge/training/train_rust_model](python-bridge/training/train_rust_model/index.md) (4 concepts)
- [python-bridge/training/prepare_grpo_dataset](python-bridge/training/prepare_grpo_dataset/index.md) (4 concepts) — Prepares GRPO RLVR rollouts from SurrealDB agent trajectories
- [python-bridge/training/train_gnn](python-bridge/training/train_gnn/index.md) (4 concepts) — Oxide-Tech-Local-Agent: Thermal & Multi-Physics Mesh Graph Neural Network (GNN)
- *…and 4 more modules*

### [scripts](scripts/index.md) — 16 concepts

- [scripts/serve_ornith](scripts/serve_ornith/index.md) (7 concepts) — Serving & Tuning Harness for Ornith-1.0-9B Q4_K_M GGUF
- [scripts/verify_ipc_bindings](scripts/verify_ipc_bindings/index.md) (4 concepts) — Verify Tauri IPC Bindings Integrity
- [scripts/find_and_bind_gguf](scripts/find_and_bind_gguf/index.md) (3 concepts) — Antigravity Autonomous GGUF File Finder & Inspector
- [scripts/test_ornith_infer](scripts/test_ornith_infer/index.md) (1 concepts)
- [scripts/run_and_tune_benchmark](scripts/run_and_tune_benchmark/index.md) (1 concepts) — Comprehensive Run & Tuning Benchmark for Ornith-1.0-9B on Oxide-Tech-Local-Agent

### [src](src/index.md) — 411 concepts

- [src/src/lib/desktop](src/src/lib/desktop/index.md) (52 concepts) — Desktop bridge: Oxide Agent Studio ↔ Tauri backend (`src-tauri`).
- [src/src/types](src/src/types/index.md) (40 concepts) — Doctor Diagnostics
- [src/src/components/ui/Dropdown](src/src/components/ui/Dropdown/index.md) (17 concepts)
- [src/src/components/KnowledgeGraphTab](src/src/components/KnowledgeGraphTab/index.md) (16 concepts)
- [src/src/components/ui/Modal](src/src/components/ui/Modal/index.md) (14 concepts)
- [src/src/components/ui/Toast](src/src/components/ui/Toast/index.md) (12 concepts)
- [src/src/design-tokens/index](src/src/design-tokens/index/index.md) (11 concepts)
- [src/src/mobile/MobileControlApp](src/src/mobile/MobileControlApp/index.md) (11 concepts)
- *…and 51 more modules*

### [src-tauri](src-tauri/index.md) — 104 concepts

- [src-tauri/src/model_ipc](src-tauri/src/model_ipc/index.md) (19 concepts)
- [src-tauri/src/memory](src-tauri/src/memory/index.md) (18 concepts) — oxide-embed memory bridge.
- [src-tauri/src/hardware_ipc](src-tauri/src/hardware_ipc/index.md) (17 concepts)
- [src-tauri/src/reforge_ipc](src-tauri/src/reforge_ipc/index.md) (13 concepts)
- [src-tauri/src/main](src-tauri/src/main/index.md) (11 concepts) — oxide-tech-local-agent — Universal Single Production Binary.
- [src-tauri/src/doctor](src-tauri/src/doctor/index.md) (8 concepts)
- [src-tauri/src/verifier_ipc](src-tauri/src/verifier_ipc/index.md) (7 concepts)
- [src-tauri/src/gateway_rt](src-tauri/src/gateway_rt/index.md) (5 concepts) — Embedded gateway runtime.
- *…and 2 more modules*

### [targets](targets/index.md) — 44 concepts

- [targets/llm-pretraining-rs/harness/src/lib](targets/llm-pretraining-rs/harness/src/lib/index.md) (22 concepts) — # Autoresearch Frozen Training Harness
- [targets/llm-pretraining-rs/surface/src/lib](targets/llm-pretraining-rs/surface/src/lib/index.md) (9 concepts) — # Autoresearch Mutable Surface (`surface.rs`)
- [targets/llm-pretraining-rs/harness/src/loader](targets/llm-pretraining-rs/harness/src/loader/index.md) (9 concepts) — # Dynamic Hot-Swap Loader with `catch_unwind` Fault Isolation
- [targets/llm-pretraining-rs/harness/src/spec](targets/llm-pretraining-rs/harness/src/spec/index.md) (4 concepts) — # Declarative Specifications & Hook Points

### [tests](tests/index.md) — 1 concepts

- [tests/e2e/antigravity_tour](tests/e2e/antigravity_tour/index.md) (1 concepts)

## Dependencies

> Full list at [`_dependencies/index.md`](/_dependencies/index.md) or `okf lookup --type Dependency`

| Ecosystem | Packages |
|----------|----------|
| cargo | 1,889 |
| npm | 332 |
| pip | 20 |
| docker | 3 |

## Key Concepts

Highest-value concepts across all domains (Classes and Functions with rich descriptions).

| Concept | Type | Module | Description |
|---------|------|--------|-------------|
| [complete_openai_compatible](/crates/vllm-client/src/client/complete_openai_compatible.md) | Function | `crates/vllm-client/src` | [tracing::instrument(name = "llm_openai_request", skip(self,… |
| [complete_ollama](/crates/vllm-client/src/client/complete_ollama.md) | Function | `crates/vllm-client/src` | [tracing::instrument(name = "llm_ollama_request", skip(self,… |
| [complete_openai_compatible](/crates/vllm-client/src/client/complete_openai_compatible_1.md) | Function | `crates/vllm-client/src` | [tracing::instrument(name = "llm_openai_request", skip(self,… |
| [complete_ollama](/crates/vllm-client/src/client/complete_ollama_1.md) | Function | `crates/vllm-client/src` | [tracing::instrument(name = "llm_ollama_request", skip(self,… |
| [generate_macro](/crates/parametric-forge/src/codegen/generate_macro.md) | Function | `crates/parametric-forge/src` | Synthesizes a `DeltaPatch` or sequence of `CadOperation`s in… |
| [generate_macro](/crates/parametric-forge/src/codegen/generate_macro_1.md) | Function | `crates/parametric-forge/src` | Synthesizes a `DeltaPatch` or sequence of `CadOperation`s in… |
| [injectStairContext](/src/src/lib/desktop/injectStairContext.md) | Function | `src/src/lib` | STAIR Code-ToC search + pack results as chat context prefix.… |
| [DynamicLoraRouter](/crates/router/src/lora_router/DynamicLoraRouter.md) | Class | `crates/router/src` | Dynamic LoRA Router: Selects task-specific LoRA weights usin… |
| [HardenedTensorMap](/crates/oxide-engines/src/mmap_tensor/HardenedTensorMap.md) | Class | `crates/oxide-engines/src` | Hardened tensor memory mapping satisfying AVX-512 and ARM NE… |
| [fetch_research](/crates/mcp-live-docs/src/perception_router/fetch_research.md) | Function | `crates/mcp-live-docs/src` | Primary intelligent perception dispatch: Tier 1 (Scrapling/D… |
| [fetch_research](/crates/mcp-live-docs/src/perception_router/fetch_research_1.md) | Function | `crates/mcp-live-docs/src` | Primary intelligent perception dispatch: Tier 1 (Scrapling/D… |
| [generate_macro](/crates/visual-forge/src/codegen/generate_macro.md) | Function | `crates/visual-forge/src` | Synthesizes a verified closed-loop CAD convergence result in… |
| [generate_macro](/crates/visual-forge/src/codegen/generate_macro_1.md) | Function | `crates/visual-forge/src` | Synthesizes a verified closed-loop CAD convergence result in… |
| [compact_compiler_log](/crates/rag-pipeline/src/okf/compact_compiler_log.md) | Function | `crates/rag-pipeline/src` | Strips excessive noise and formats compiler outputs to prese… |
| [compact_compiler_log](/crates/rag-pipeline/src/okf/compact_compiler_log_1.md) | Function | `crates/rag-pipeline/src` | Strips excessive noise and formats compiler outputs to prese… |
| [DeltaPatch](/crates/parametric-forge/src/patcher/DeltaPatch.md) | Class | `crates/parametric-forge/src` | A fine-grained delta patch that modifies only a masked hiera… |
| [new](/crates/rag-pipeline/src/lib/new.md) | Function | `crates/rag-pipeline/src` | Initialize the RAG pipeline by connecting to Qdrant, initial… |
| [new](/crates/rag-pipeline/src/lib/new_1.md) | Function | `crates/rag-pipeline/src` | Initialize the RAG pipeline by connecting to Qdrant, initial… |
| [SglangProvider](/crates/vllm-client/src/sglang_provider/SglangProvider.md) | Class | `crates/vllm-client/src` | High-throughput SGLang inference provider using RadixAttenti… |
| [query_ast_netlist_coupling](/crates/rag-pipeline/src/graph_rag/query_ast_netlist_coupling.md) | Function | `crates/rag-pipeline/src` | Query hardware-software cross-domain coupling (AST symbol ma… |

## Usage with OpenCode

```bash
# Prime full context
RUN cat ./okf_bundle/SUMMARY.md

# Prime specific domain
RUN cat ./okf_bundle/crates/index.md

# Find a concept
RUN find ./okf_bundle -name '<ConceptName>.md' | xargs cat
```
