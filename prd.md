# Product Requirements Document (PRD): Oxide-Tech Local Agent OS

**Document Status:** Complete & Authoritative  
**Product Specification:** High-Performance Sovereign Autonomous Engineering Operating System  
**System Target:** Single-Binary Hermetic Desktop Monolith (`oxide-tech-local-agent`) & Unified Substrates  
**Core Lead & Architect:** Faez Barghasa (Senior Freelance Embedded & Systems Engineer)  
**Host Target:** Pop!_OS 24.04 LTS (Cosmic / Linux x86_64) | Bare-Metal & SWD Targets: STM32F401, STM32F407, STM32F103, ESP32-S3  
**Strategic Mandate:** 100% Offline-First, Local-First, Token-Lean, Zero-Cloud Egress, Deterministic Verification, Mathematical Rigor, and Zero-Python Runtime.

---

## 1. Executive Summary & Vision

### 1.1 The Problem
Modern AI coding assistants and agent frameworks suffer from compounding systemic defects:
1. **Cloud & Token Bleeding:** Massive context-window pollution, round-trip latency (1.5s–5s), high recurring token costs, and catastrophic fragility when network connectivity is severed or throttled.
2. **Fragile Polyglot Glue:** Sprawling Python scripts, brittle venv/pip dependencies, loose IPC serialization, and runtime crash points (`AttributeError`, `NoneType`) in safety-critical engineering domains.
3. **Hallucinatory Code Generation:** LLMs output syntactically plausible but electrically hazardous, mathematically invalid, or concurrency-broken code (e.g. unbuffered SPI DMA races, deadlocks, thermal runaway in copper traces).
4. **Scattered Toolchains:** Fragmented workflows across separate IDEs, external CLI tools, standalone terminal servers, and hardware programmers.

### 1.2 The Sovereign Solution: Oxide-Tech Local Agent OS
Oxide-Tech Local Agent OS is an autonomous, single-binary, hermetic desktop engineering workstation written in **pure modern Rust (Edition 2024, 1.85+)**. It unifies:
- **Desktop Monolith Architecture:** A single compiled native executable (`oxide-tech-local-agent`) delivering the Tauri v2 desktop GUI with inlined React 19 / Tailwind CSS assets (`rust-embed`), an in-process background Actix-Web + Quinn QUIC gateway server (`:8080`), local GGUF tensor execution, and hardware probe integration.
- **STAIR (Structure-Aware Information Retrieval):** Hierarchical Tree-Sitter Code-ToC leaf routing with enclosing lexical breadcrumbs, cutting prompt token consumption by up to 94% with zero semantic bleeding.
- **Memanto Semantic Memory Fabric:** GraphRAG, 13 typed memory categories (`decision`, `instruction`, `fact`, `commitment`, etc.), temporal time travel, and automated contradiction resolution.
- **Pure-Rust Multimodal Forges:** On-device STT audio transcription and streaming ring buffers (`audio-forge`), local diffusion media generation (`media-forge`), and native binary / PTX GPU reverse engineering (`re-forge`).
- **Closed-Loop Co-Design & Formal Verification:** SMT-LIB2 circuit safety invariants, IPC-2152 thermal current dissipation calculations, SWD hardware-in-the-loop flashing (`mcp-probe-rs`), and automated git-stash rollback checkpointers.

---

## 2. User Persona & Engineering Constraints

### 2.1 Primary Persona: Faez Barghasa
- **Profile:** Senior Freelance Embedded & Systems Engineer based in Iran.
- **Operating Reality:** Requires reliable, high-speed, local-first engineering capabilities that function seamlessly without constant high-bandwidth internet access. Token economy and compute efficiency are paramount.
- **Core Technology Stack:**
  - *Embedded:* `no_std`, RTIC v2, Embassy async, `defmt`, `postcard`, `heapless`, STM32F401, STM32F407, STM32F103, ESP32-S3.
  - *Desktop CAD & HMI:* Iced 0.14+ (`wgpu`), Slint, Dioxus, Tauri v2.
  - *Systems & Network:* Actix-Web, Tonic gRPC, Quinn QUIC, rustls with `aws-lc-rs`, SurrealDB v3, Qdrant.
  - *Memory & Context:* `oxide-embed`, Tree-Sitter, FastEmbed.
- **Hard Behavioral Rules:**
  - Zero `.unwrap()` in production paths.
  - Zero unrequested abstractions (strict Ponytail YAGNI principles).
  - Concise diff-style outputs; zero introductory or concluding conversational filler.
  - Hardware watchdogs, safe-state failovers, and mathematical proofs before flashing.

---

## 3. Product Architecture & Subsystem Specification

### 3.1 Architectural Manifold

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        TAURI V2 DESKTOP MONOLITH SHELL                                │
│                   React 19 + Tailwind CSS Studio (Inlined rust-embed)                   │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                           CONSOLIDATED IPC CONTROLLER FACADE                           │
│  - AgentController    - SystemController    - WorkspaceController    - ForgeController  │
├───────────────────────────────────┬────────────────────────────────────────────────────┤
│   IN-PROCESS GATEWAY & NETWORKING │       COGNITIVE AGENT ORCHESTRATOR (RIG-CORE)      │
│  - Actix-Web HTTP/1.1 & HTTP/2    │  - Rig Agent Prompt & Completion Loops             │
│  - Quinn QUIC / HTTP3 Datagrams   │  - MoE Gating Router (Sub-10ms SLA)                │
│  - 19 Algorithmic Route Policies  │  - Native Rust Tool Calling Interface              │
│  - 128-bit Anti-Replay Wire Frame │  - Dynamic VRAM Model Admission Controller         │
├───────────────────────────────────┴────────────────────────────────────────────────────┤
│                           DATA & SEMANTIC MEMORY FABRIC                                │
│  - oxide-embed (STAIR Code-ToC Hierarchical Retrieval & Memanto GraphRAG)              │
│  - Embedded SurrealDB v3 (SurrealKV In-Memory / Local Storage Engine)                  │
│  - Embedded Qdrant Vector Engine & FastEmbed Micro-Embeddings                          │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                       PURE-RUST MULTI-DOMAIN FORGES & HARDWARE                         │
│  - audio-forge: Ring Buffer Audio Capture & Speech-to-Text Transcription               │
│  - media-forge: FlowMatch Euler Diffusion Pipeline & Diagram Synthesizer               │
│  - re-forge: PTX GPU Disassembly, Cortex-M Vector Tables & Shannon Entropy             │
│  - mcp-probe-rs: SWD Flash Manager with Hardware Watchdog & Safe-State Rollback        │
│  - oxide-kernels: SIMD AVX-512 Guarded Memory & Ternary Bitplane Matrix Kernels        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Detailed Functional Requirements & Feature Sets

### 4.1 Single-Binary Desktop Workstation
- **FR-01 (Single Binary Entrypoint):** The repository MUST build into a single native binary (`target/release/oxide-tech-local-agent`). Running `cargo run` or executing the binary without arguments launches the native desktop GUI.
- **FR-02 (Subcommand Matrix):** The binary MUST support headless and administrative execution modes:
  - `desktop [--config PATH]`: Native Tauri v2 window with inlined web assets and embedded gateway.
  - `--install [--force]`: Automated installation of desktop menu entries, udev rules, and binary paths.
  - `daemon [--config PATH]`: Headless gateway foreground service (systemd compatible).
  - `doctor [--json]`: Comprehensive diagnostics verifying toolchains, udev permissions, `probe-rs`, GGUF paths, and database health.
  - `re-forge <FILE> [--arch ARCH] [--json]`: Disassembles ELF, Mach-O, PE, or PTX GPU code.
  - `verify [--workspace PATH] [--json]`: Runs the deterministic verification suite and exports signed cryptographic evidence bundles.
  - `memory <SUBCOMMAND> [ARGS...]`: Passthrough to STAIR Code-ToC leaf search, AST outline, and Memanto semantic memories.
  - `status [--gateway-url URL]`: Gateway health and liveness probe.
- **FR-03 (Zero-Blocking Async Runtime):** All heavy disk traversals, synchronous GGUF inspections, and process executions MUST be dispatched through `tokio::task::spawn_blocking` to protect Tokio async worker threads.
- **FR-04 (Global Memory Allocator):** Production builds MUST use `mimalloc` to prevent memory fragmentation during high-throughput tensor operations and audio streaming.

### 4.2 Multi-Provider Inference & Dynamic VRAM Admission
- **FR-05 (Native GGUF Lifecycle Supervision):** The system MUST automatically detect and supervise `llama-server` on port `8081` with GPU layer offloading (`-ngl 99`), context management, and PID watchdog lifecycle handling.
- **FR-06 (Dynamic VRAM Admission Engine):**
  - Query GPU total, free, and used VRAM in real time via `sysinfo` and `/bin/nvidia-smi`.
  - Maintain a mandatory 1.5 GB safety headroom for display servers, window managers, and KV cache.
  - Dynamically calculate safe `-ngl` GPU offload layers or automatically fall back to CPU system RAM to eliminate Out-Of-Memory (OOM) hard crashes.
- **FR-07 (Recursive Weight Auto-Discovery):** Automatically scan and index local `.gguf` weights across `~/models`, `~/.cache/huggingface`, `~/.lmstudio/models`, `~/.ollama/models`, and `/opt/models`.
- **FR-08 (19 Universal Gateway Routing Strategies):** The gateway MUST support:
  `priority`, `fill-first`, `weighted`, `round-robin`, `p2c`, `least-used`, `random`, `strict-random`, `cost-optimized`, `headroom`, `reset-window`, `reset-aware`, `context-relay`, `context-optimized`, `cache-optimized`, `lkgp`, `auto`, `fusion`, and `chaos`.

### 4.3 Context Reduction & Project Memory (STAIR & Memanto)
- **FR-09 (STAIR Code-ToC Hierarchical Search):** Use Tree-Sitter AST slicing to index files by bounded macro nodes (traits, structs, impl blocks) and leaf functions. Injections into agent context MUST include file breadcrumbs and exact AST bounds.
- **FR-10 (Memanto Typed Semantic Memory):**
  - Support 13 typed categories: `instruction`, `fact`, `decision`, `goal`, `commitment`, `preference`, `relationship`, `context`, `event`, `learning`, `observation`, `artifact`, `error`.
  - Automated polarity conflict resolution (auditing contradictions via `oxide-embed conflicts`).
  - Temporal time-travel querying (`--as-of <ISO-TIMESTAMP>`).
- **FR-11 (Token Budget Packing):** Context generation MUST enforce hard token ceilings (e.g. `--budget 1500`) using knapsack packing to prevent token overflow.

### 4.4 Pure-Rust Audio & Multimodal Forges
- **FR-12 (audio-forge Streaming Ring Buffer):** Low-latency non-blocking audio capture with dedicated circular buffers, real-time RMS power tracking, and local speech-to-text transcription engine.
- **FR-13 (media-forge FlowMatch Diffusion):** Local image and diagram generation using Euler FlowMatch scheduling without external Python dependencies.
- **FR-14 (re-forge Binary Reverse Engineering):** Disassemble bare-metal ARM Cortex-M firmware, parse vector tables, compute Shannon entropy across sections, and decode PTX GPU kernel code.

### 4.5 Embedded Hardware & Electronics Co-Design
- **FR-15 (mcp-probe-rs SWD Management):** Direct hardware debugging and flashing over SWD/JTAG for STM32 and Nordic microcontrollers with atomic flash verification and auto-rollback on verification failure.
- **FR-16 (Circuit Thermal & Safety Solvers):** Enforce IPC-2152 compliant maximum current calculations on PCB copper traces and formal SMT-LIB2 invariant checks before generating fabrication netlists.

---

## 5. UI/UX Design System & Aesthetic Standard

### 5.1 Design Philosophy: Sovereign Craftsmanship
The desktop UI adheres to the **Better-Design**, **Tasteful-UI**, and **UI/UX Design** frameworks:
- **Zero AI Slop:** No emoji-as-icons, no floating unaligned containers, no low-contrast illegible text.
- **High-Density Technical HUD:** Inspired by precision CAD/EDA workstations and avionics telemetry.

### 5.2 Color Tokens & Surface Elevation
- **`bg-base` (`#090D12`):** Ultra-dark void background minimizing eye fatigue during late-night systems engineering.
- **`bg-surface` (`#131B24`):** Secondary panel background with subtle glassmorphic depth (`backdrop-blur-md`).
- **`bg-elevated` (`#1A2432`):** Elevated cards, modals, and popovers with 1px border strokes (`border-white/10`).
- **`accent-amber` (`#E5A93C`):** Sovereign amber highlighting hardware warnings, active SWD flash status, and critical alerts.
- **`accent-cyan` (`#38BDF8`):** Cyber cyan highlighting active LLM reasoning streams, STAIR Code-ToC breadcrumbs, and memory links.
- **`text-primary` (`#E6EDF3`):** High-contrast crisp typography.
- **`text-muted` (`#8B949E`):** Secondary meta tags, timestamps, and register addresses.

### 5.3 Typography & Spatial Rhythm
- **Grid:** Strict 4px base with 8px / 12px / 16px / 24px component padding.
- **Font Stack:**
  - *Branding & Headers:* Space Grotesk (geometric, technical authority).
  - *Body & Controls:* Inter (ultra-legible, neutral, dense).
  - *Telemetry & Code:* JetBrains Mono (fixed-width tabular figures, memory addresses, hex dumps).

---

## 6. Precondition-Gated Capability Horizons (Sovereign Master Plan)

State progression is strictly time-invariant and governed exclusively by mathematical soundness and formal test verification:

| Horizon | Scope | Entry Precondition | Verification Gate |
|---|---|---|---|
| **Horizon 0** | Monolith Desktop Foundation | Clean workspace topology | `pure_rust_stack_test` & `cargo clippy -- -D warnings` |
| **Horizon I** | Single-Binary Hermetic Packaging | Inlined web assets (`rust-embed`) | Single binary runs desktop + headless daemon |
| **Horizon II** | Non-Blocking Async & GGUF Supervision | `tokio::task::spawn_blocking` | Realtime UX test passed (<10ms routing SLA) |
| **Horizon III** | Closed-Loop EDA & SPICE Co-Simulation | KiCad PCB & Netlist parser | IPC-2152 thermal & DRC 0 errors |
| **Horizon IV** | Mechanical CAD B-Rep & FEA Solids | `rapier3d` / `parry3d` integration | Euler-Poincaré topological manifold valid |
| **Horizon V** | Sovereign P2P Overlay Mesh Fabric | Quinn QUIC + Blake3 AEAD | Zero-cloud relay egress & biometric pass |
| **Horizon VI** | Self-Evolution & Policy Distillation | GRPO reward harvesting | Sandboxed Wasm tool synthesis verified |
| **Horizon VII** | Heterogeneous Workstation Swarm | CRDT consensus lattice | Autonomous co-design & Gerber CAM release |

---

## 7. Non-Functional & Quality Assurance Requirements

### 7.1 Performance Metrics
- **Desktop Cold Startup:** $\le 350\text{ ms}$ to interactive native window.
- **MoE Gating Routing Latency:** $\le 10\text{ ms}$ under concurrent load (25 simulated users, 1,000 requests).
- **STAIR AST Surgical Slicing:** $\le 50\text{ }\mu\text{s}$ per leaf function slice.
- **Memory Footprint:** Headless daemon idle RAM $\le 85\text{ MB}$; Desktop GUI idle RAM $\le 190\text{ MB}$.

### 7.2 Safety & Reliability Invariants
- **Zero Panic Guarantee:** Zero `.unwrap()` in production paths. All external I/O and FFI boundaries MUST return strongly typed `Result<T, E>`.
- **Fault-Isolated Execution:** Subprocesses and plugin sandboxes execute within Linux namespaces (`bubblewrap` / `nix`).
- **Hardware Interlocks:** SWD flash commands require pre-flight checksum verification and automated bootloader partition rollback on failure.

---

## 8. Verification & Acceptance Checklist

To verify that the system satisfies this PRD:

```bash
# 1. Verify workspace code compiles cleanly with default desktop monolith target
cargo check

# 2. Enforce zero clippy warnings across all workspace members and test targets
cargo clippy --workspace --all-targets -- -D warnings

# 3. Execute pure-Rust single-binary hermetic stack test suite
cargo test --test pure_rust_stack_test -- --nocapture

# 4. Execute real-time concurrency and user experience SLA test suite
cargo test --test realtime_user_experience_test -- --nocapture

# 5. Run full workspace integration test matrix
cargo test --workspace

# 6. Verify oxide-embed AST diagnostics and project health
oxide-embed doctor
```
