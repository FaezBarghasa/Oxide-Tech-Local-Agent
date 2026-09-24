# Oxide-Tech Local Agent OS: System Architecture (v2.2)

**Oxide-Tech-Local-Agent** is organized into 6 primary engineering layers, ensuring full isolation, structural intelligence, deterministic verification, autonomous self-evolution, and multi-domain physical co-simulation.

---

## 1. Six-Layer Architecture & Data Flow

```mermaid
graph TD
    Client[User / IDE / Studio UI: React 19] -->|TCP JSON-RPC / UDP HTTP3 / SSE| Gateway[Gateway: Actix + Quinn QUIC 8080]

    subgraph DTXLayer [Distributed Transaction Layer: The Oxide Protocol]
        DTX[DtxCoordinator: UUIDv7 DTX State Machine]
        DTX -->|Propagates dtx_id| MCP[Local MCP Client Transports]
    end

    subgraph Layer0 [Layer 1: External Research & Perception Layer]
        Router[Dynamic Perception Dispatcher]
        Tier1[Tier 1: d4vinci/Scrapling (Local Fast Headless)]
        Tier2[Tier 2: pinchtab/pinchtab (Local Interactive Daemon)]
        Tier3[Tier 3: Cloudflare Kitesurf (Cloud V8 Browser Isolates)]
        Router --> Tier1
        Router --> Tier2
        Router --> Tier3
    end

    subgraph Layer1 [Layer 2: Graph Engineering & Multi-Domain Memory]
        AST[Tree-sitter AST Extractor] --> NodeGraph[Multi-Modal Code Graph]
        NodeGraph --> Impact[Impact Blast Radius Analyzer]
        Packer[CrossDomainContextPacker: Firmware AST + PCB Netlist + CAD Mesh]
    end

    subgraph Layer2 [Layer 3: Context & Memory Layer]
        RingBuffers[Ephemeral Ring Buffers: Terminal / Editor / Logs]
        TemporalGit[Temporal Git Churn & Co-Change Coupling]
        SurrealGraph[SurrealDB v3 Property Graph: code_symbol -> maps_to -> eda_component]
        QdrantVec[Qdrant Hybrid Semantic Index]
    end

    subgraph Layer3 [Layer 4: Agent Engineering Layer]
        Supervisor[Supervisor Agent Swarm]
        Persona[Persona DAG: Researcher -> Architect -> Coder -> DRC]
        LoRARouter[Dynamic LoRA Adapter Hot-Swap Router]
    end

    subgraph Layer4 [Layer 5: Loop & Execution Layer]
        Observer[Meta-Cognitive Observer & Scorer]
        Oscillation[Oscillation Loop Guard]
        Checkpoint[Atomic Checkpointer: git stash create]
        Verifiers[Deterministic Verifiers: cargo, kicad, qemu]
        CoSim[cross-domain-verifier: Electro-Thermal-Mechanical Co-Simulation]
    end

    subgraph Layer5 [Layer 6: Self-Evolution Engine]
        DeltaHarv[DeltaHarvester: Compiler Diff Collector]
        TrainingPool[SurrealDB grpo_training_pool]
        ToolMaker[JIT MCP Tool Synthesizer: bwrap Sandbox]
        SkillOpt[SkillOpt Crystallizer -> SKILL.md]
    end

    Gateway --> DTXLayer
    DTXLayer --> Layer0
    Layer0 --> Layer1
    Layer1 --> Layer2
    Layer2 --> Layer3
    Layer3 --> Layer4
    Layer4 --> Layer5
```

---

## 2. Non-Autoregressive Pure-Rust Decision Engine (`crates/oxide-engines`)

The decision engine runs **strictly non-autoregressive** (single forward pass, deterministic output, no token-generation loop), achieving ultra-low latency, calibrated confidence, and sub-500 MB footprint:

```text
[ Caller Threads / Rayon Tasks ]
              │
              ▼ (Non-blocking send via Flume MPMC Channel)
    ┌────────────────────────┐
    │     Job Queue          │
    └────────────────────────┘
              │
              ▼ (Dynamic Micro-batching: up to 32 jobs or 2ms timeout)
    ┌──────────────────────────────────┐
    │ Dedicated Worker Thread          │ ──► [Non-Autoregressive Decision]
    │ (CandidateVectorCache / Fast-KAN)│     • Single pooled forward pass
    └──────────────────────────────────┘     • Brier score calibration
              │
              ▼ (Scatter results via oneshot sync channels)
[ Return Result: { selected, confidence, latency } ]
```

### Core Primitives:
- **`DecisionEngine`**: Zero-mutex contention actor-worker handle executing batched matrix multiplications on a dedicated thread.
- **`CandidateVectorCache`**: Pre-computed normalized candidate embeddings for 30–100+ choices, performing microsecond dense dot-product evaluations against state embeddings.
- **`FastKanDecisionHead`**: Non-autoregressive B-spline/trigonometric activation projection resolving multi-candidate probability distributions in a single pass.
- **`BrierScoreLoss`**: Mathematically calibrated loss function avoiding the artificial overconfidence of standard cross-entropy.
- **`AlignedTensorMap`**: SIMD-aligned (64-byte boundary) memory-mapped weights with `fs2` advisory reader locks.

---

## 3. Subsystem Deep Dive

### Layer 1: External Research & Perception Layer (`crates/mcp-live-docs`)
- **Stealth Local Extraction (`d4vinci/Scrapling`)**: Fast Python-based extraction worker handling `docs.rs`, GitHub issues, and crates.io with sub-200ms DOM parsing.
- **Local Interactive Browser Daemon (`pinchtab/pinchtab`)**: Lightweight Go daemon delivering accessibility-tree snapshots and Cloak Mode automation.

### Layer 2 & 3: Multi-Domain Graph & Unified Memory (`crates/knowledge` & `crates/memory`)
- **Cross-Domain Topology Mapping**:
  - `code_symbol -> maps_to -> eda_component`
  - `eda_component -> mates_with -> cad_body`
  - `code_function -> constrains -> cad_feature`
- **Cross-Domain Context Packing (`cross_domain_packer.rs`)**: Packs Firmware AST, Electronics Netlists, and 3D CAD feature trees within LLM token budgets.

### Layer 4: Multi-Physics Co-Simulation & Verification (`crates/cross-domain-verifier` & `crates/forge-rust`)
- **Electro-Thermal-Mechanical Loop**:
  1. Computes dynamic MCU wattage from firmware duty cycles.
  2. Evaluates PCB power density across copper layers.
  3. Simulates thermal dissipation over CAD enclosure meshes and material conductivities (Aluminum 6061, PETG, ABS).
  4. Automatically triggers heatsink fin and thermal via generation if silicon junction temperature exceeds $85^\circ\text{C}$.
- **Polyglot Refactoring & Synthesis (`crates/forge-rust`)**:
  - Ingests foreign language codebases (C/C++, Python, TypeScript, Go, Java, Generic).
  - Normalizes syntax into a Polyglot Universal Intermediate Representation (UIR).
  - Refactors memory layouts, error propagation, and concurrency models to safe, idiomatic Rust 2024.
  - Automatically verifies syntax with `syn` and scaffolds complete Cargo workspaces.

### Layer 5 & 6: Distributed Transactions & Self-Evolution (`crates/oxide-protocol` & `crates/self-evolver`)
- **The Oxide Protocol**: Standardized JSON-RPC 2.0 schemas for EDA and 3D CAD tools with time-ordered **UUIDv7 Distributed Transaction IDs (`DtxId`)** and automatic atomic rollbacks (`rollback_dtx`).
- **Self-Evolution Engine**: `DeltaHarvester` and JIT MCP synthesizer running inside unshared `bwrap` namespaces.

### Layer 7: Agent Runtime Safety & Process Supervision (`crates/oxide-security` & `crates/oxide-state`)
- **Circuit Breaker Resource Gater (`resource_gater.rs`)**: Continuous polling of storage and VRAM headroom ($< 2\text{GB}$ freeze / $> 2.5\text{GB}$ resume hysteresis). Rejects incoming HTTP/QUIC requests before OS disk/VRAM exhaustion occurs.
- **Idempotent Session Lifecycle Machine (`session_supervisor.rs`)**: Enforces atomic fsync `SessionReceipt` persistence, request deduplication, and a single crash recovery attempt guarantee.
- **Secure Stderr Capture & Sanitization (`stderr_sanitizer.rs`)**: 16 KiB bounded ring buffer tail capture with regex scrubbing of API keys/tokens into `0600` root-isolated diagnostic files.
- **Process Group Containment (`process_containment.rs`)**: Enforces POSIX process group tree (`setpgid`) wrapping with 4-second SIGTERM grace intervals and SIGKILL tree destruction to eliminate zombie processes.

### Layer 8: Accelerated Compute & Tiered Memory Architecture (`crates/oxide-kernels` & `crates/model-trainer`)
- **GPU Architecture Autotuning (`autotune.rs`)**: Hardware SM detection across Nvidia Ampere (SM80/86), Ada (SM89), Hopper (SM90), and Blackwell (SM100/120) with optimal tile sizing and warp allocations.
- **DDR5 Host RAM Spillover Tier (`ddr5_offload.rs`)**: Tiered hierarchy (`GpuVram` $\to$ `HostDdr5` $\to$ `NvmeDisk`) automatically evicting tensors to pinned host RAM when VRAM headroom drops below $800\text{ MB}$.
- **AVX-512 Tensor Compression (`avx512_compress.rs`)**: 4x memory bandwidth reduction converting FP32 tensors to INT8 with dynamic scaling factors via AVX-512F/BW SIMD vectorization.

---

## 3. Desktop-First Integration (`src-tauri` & `ui/oxide-agent-studio`)

The desktop application directly mounts the full suite of backend capabilities across specialized studio views:
- **System Doctor**: Direct target connectivity, hardware permissions, and udev rule deployment.
- **RE-Forge Studio**: Binary architecture analysis, ARM vector table parsing, entropy graphs, and safe-Rust decompilation.
- **Verification Matrix**: Real-time multi-suite verification execution, checkpointer rollback, and evidence bundle generation.
- **Memory & Rule Fabric**: Persistent decision inspection, contradiction detection, and 2-hop GraphRAG traversals.
- **Settings & Profile Manager**: Real-time TOML profile switching (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`).
