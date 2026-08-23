# Oxide-Tech Local Agent OS: System Architecture (v2.1)

**NexusForge / Oxide-Tech-Local-Agent** is organized into 6 primary engineering layers, ensuring full isolation, structural intelligence, deterministic verification, autonomous self-evolution, and deep internet-connected perception.

---

## 1. Six-Layer Architecture & Data Flow

```mermaid
graph TD
    Client[User / IDE / Studio UI: React 19] -->|TCP JSON-RPC / UDP HTTP3 / SSE| Gateway[Gateway: Actix + Quinn QUIC 8080]

    subgraph Layer0 [Layer 1: External Research & Perception Layer (Eyes & Ears)]
        Router[Dynamic Perception Dispatcher]
        Tier1[Tier 1: d4vinci/Scrapling (Local Fast Headless)]
        Tier2[Tier 2: pinchtab/pinchtab (Local Interactive Daemon)]
        Tier3[Tier 3: Cloudflare Kitesurf (Cloud V8 Browser Isolates)]
        Router --> Tier1
        Router --> Tier2
        Router --> Tier3
    end

    subgraph Layer1 [Layer 2: Graph Engineering Layer (Structural Skeleton)]
        AST[Tree-sitter AST Extractor] --> NodeGraph[Multi-Modal Code Graph]
        NodeGraph --> ExtDep[External Dependency Node Injector]
        NodeGraph --> Impact[Impact Blast Radius Analyzer]
        NodeGraph --> Pruner[Graph-Guided Context Slicer -80% Tokens]
    end

    subgraph Layer2 [Layer 3: Context & Memory Layer (Neural Cortex)]
        RingBuffers[Ephemeral Ring Buffers: Terminal / Editor / Logs]
        TemporalGit[Temporal Git Churn & Co-Change Coupling]
        SurrealGraph[SurrealDB v3 Property Graph]
        QdrantVec[Qdrant Hybrid Semantic Index with Provenance]
    end

    subgraph Layer3 [Layer 4: Agent Engineering Layer (Adaptive Brain)]
        Supervisor[Supervisor Agent Swarm]
        Persona[Persona DAG: Researcher -> Architect -> Coder -> DRC]
        LoRARouter[Dynamic LoRA Adapter Hot-Swap Router]
    end

    subgraph Layer4 [Layer 5: Loop & Execution Layer (Execution Engine)]
        Observer[Meta-Cognitive Observer & Scorer]
        Oscillation[Oscillation Loop Guard]
        Checkpoint[Atomic Checkpointer: git stash create]
        Verifiers[Deterministic Verifiers: cargo, kicad, qemu]
    end

    subgraph Layer5 [Layer 6: Self-Evolution Engine]
        DeltaHarv[DeltaHarvester: Compiler Diff Collector]
        TrainingPool[SurrealDB grpo_training_pool]
        ToolMaker[JIT MCP Tool Synthesizer: bwrap Sandbox]
        SkillOpt[SkillOpt Crystallizer -> SKILL.md]
    end

    Gateway --> Layer0
    Layer0 --> Layer1
    Layer1 --> Layer2
    Layer2 --> Layer3
    Layer3 --> Layer4
    Layer4 --> Layer5
```

---

## 2. Subsystem Deep Dive

### Layer 1: External Research & Perception Layer (`crates/mcp-live-docs` & `python-bridge`)
- **Tier 1: Stealth Local Extraction (`d4vinci/Scrapling`)**: Fast Python-based extraction worker handling `docs.rs`, GitHub issues, and crates.io with sub-200ms DOM parsing and built-in anti-fingerprinting.
- **Tier 2: Local Interactive Browser Daemon (`pinchtab/pinchtab`)**: Lightweight Go HTTP server (~12MB) delivering full accessibility-tree snapshots, multi-tab orchestration, and Cloak Mode automation without cloud costs.
- **Tier 3: Cloud Agent-First Browser Engine (`Cloudflare Kitesurf / Browser Run`)**: Stateless V8 isolate engine for parallelized deep web research, heavy JS SPAs, Turnstile bypass, and offloaded visual UI / PDF verification.

### Layer 2: Graph Engineering Layer (`workspace/knowledge`)
- **AST & Call Graph Parser (`ast.rs`)**: Tree-sitter powered parser extracting typed `CodeGraphNode` (Files, Modules, Structs, Traits, Functions, Fields, Variables, `ExternalDependency`, `ExternalDoc`) and `CodeGraphEdge` (Defines, Calls, Implements, References, DataFlowsTo, Imports, `DependsOnExternal`).
- **Impact Analysis (`impact_analysis.rs`)**: $K$-hop topological BFS determining downstream breaking blast radiuses, affected files, and recommended test targets.
- **Subgraph Slicer (`subgraph_pruner.rs` & `context_slicer.rs`)**: Extracts minimal 1-hop and 2-hop topological subgraphs as JSON slices to reduce LLM prompt tokens by 75%–89%.

### Layer 3: Context & Memory Layer (`workspace/memory`)
- **Ephemeral Buffers (`ephemeral.rs`)**: Lock-free concurrent ring buffers capturing terminal stream logs, active editor dirty buffers, and panic stack traces.
- **Temporal History (`temporal_git.rs`)**: Analyzes git commit churn rates and file-pair co-change coupling matrices.
- **Dual-State Persistence (`surrealdb-service` & `qdrant-service`)**: SurrealDB v3 property graph schema coupled with high-dimensional Qdrant vector points with strict research provenance (URL, engine, timestamp, confidence).

### Layer 4: Agent Engineering Layer (`workspace/router` & `crates/optio`)
- **Multi-Persona Supervisor (`supervisor.rs` & `persona_loop.rs`)**: Dynamic DAG task decomposition and delegation across personas (`Lead Architect`, `Researcher`, `Bare-Metal Firmware Coder`, `EDA Schematic Engineer`, `DRC Reviewer`).
- **Dynamic LoRA Hot-Swapper (`lora_router.rs`)**: Real-time task intent prediction and adapter activation (`lora_embedded_rust_v2`, `lora_kicad_schgen_v3`, `lora_cad_b3d_v1`) via SGLang runtime `/v1/lora/activate`.

### Layer 5: Loop & Execution Layer (`crates/optio` & `workspace/verifier`)
- **Meta-Cognitive Observer (`observer.rs`)**: Evaluates token generation loop efficiency, flags hallucinations, and stops redundant repetitive tool calls.
- **Oscillation Guard (`oscillation.rs`)**: Prevents circular repair loops and infinite retry states.
- **Atomic Checkpointer (`checkpoint.rs`)**: Captures instantaneous working tree states (`git stash create`) and performs instant rollbacks on verification failures.

### Layer 6: Self-Evolution Engine (`crates/self-evolver`)
- **Delta-RL Harvester (`delta_harvester.rs`)**: Computes unified text diffs between failed code and compiler-verified fixes, logging training pairs to `grpo_training_pool` in SurrealDB.
- **JIT MCP Tool Synthesizer (`tool_maker.rs`)**: Generates standalone Python/Mojo tools and tests them inside unshared Linux namespace sandboxes using Bubblewrap (`bwrap`).
- **SkillOpt Workflow Crystallizer (`skill_crystallizer.rs`)**: Distills verified multi-turn execution trajectories into reusable `SKILL.md` workflows under `workspace/skills/`.

---

## 3. Dual-GPU Hardware Offload Architecture

| Device | Role | VRAM Allocation |
|---|---|---|
| **GPU 0 (PCIe 4.0 x16)** | SGLang Tensor Parallel Rank 0 (Qwen3.8-35B), Fast Intent Router (Gemma-4-9B / Ornith-1.0), Mojo SIMD Vector Engine | ~22 GB |
| **GPU 1 (PCIe 4.0 x16)** | SGLang Tensor Parallel Rank 1 (Qwen3.8-35B), Dynamic LoRA Adapter Pool, SF3D / B-Rep Latent Geometry Diffusion Engine | ~22 GB |
