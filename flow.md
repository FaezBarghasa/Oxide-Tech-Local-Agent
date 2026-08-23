# Oxide-Tech Local Agent OS: System Control Flow (v2.1)

This document details the step-by-step logic, runtime control loops, and execution algorithms that govern Oxide-Tech Local Agent OS.

---

## 1. Tri-Engine Perception & External Research Flow

```
[Research / Perception Intent Identified]
                   │
                   ▼
[PerceptionRouter (perception_router.rs)]
  - Evaluates target URL domain & task requirements
                   │
  ┌────────────────┼────────────────────────┐
  │                │                        │
  ▼                ▼                        ▼
[Tier 1: Scrapling] [Tier 2: PinchTab]   [Tier 3: Kitesurf]
- Python stealth    - Local Go daemon    - Cloudflare V8 isolates
- Sub-200ms DOM     - AX-tree snapshots  - Anti-bot / Turnstile bypass
- Docs & Crates.io  - Interactive click  - Parallel deep research
  │                │                        │
  └────────────────┴────────────────────────┘
                   │
                   ▼
[Grounded Memory Ingestion (client.rs)]
  - Embeds chunks into Qdrant `rust_rag`
  - Records provenance (URL, engine, timestamp, confidence >= 0.80)
  - Injects `ExternalDependencyNode` into Multi-Modal Code Graph
```

---

## 2. Multi-Modal Graph Traversal & Context Pruning Flow

```
[Target Node Modified (Function / Struct / Net / ExternalDependency)]
                    │
                    ▼
[AST & Tree-sitter Extractor (ast.rs)]
  - Parses file syntax, identifiers, spans, doc comments
  - Generates CodeGraphNode and CodeGraphEdge
                    │
                    ▼
[Topological Impact Analysis Engine (impact_analysis.rs)]
  - Executes K-hop BFS on reverse 'calls' / 'references' edges
  - Computes blast radius, affected downstream files, and recommended tests
                    │
                    ▼
[Graph-Guided Context Slicer (context_slicer.rs)]
  - Isolates 1-hop callers and 2-hop type signatures
  - Serializes minimal JSON subgraph slice
  - Reduces LLM context token usage by 75%–89%
```

---

## 3. Dynamic LoRA Adapter Hot-Swapping Flow

1. **Intent Analysis**: The user prompt is analyzed by `LoraRouter` in `workspace/router/src/lora_router.rs`.
2. **Domain Classification**:
   - `FirmwareEmbedded` (`no_std`, `stm32`, `dma`, `uart`) $\rightarrow$ activates `lora_embedded_rust_v2`.
   - `PcbCad` (`kicad`, `schematic`, `netlist`, `skidl`) $\rightarrow$ activates `lora_kicad_schgen_v3`.
   - `CAD3D` (`blender`, `b-rep`, `step`, `opencascade`) $\rightarrow$ activates `lora_cad_b3d_v1`.
3. **Runtime Activation**: Sends HTTP POST to `/v1/lora/activate` on SGLang serving cluster without restarting weights.

---

## 4. Auto-Healing & Verification Loop Flow

1. **Working Tree Snapshot**: `CheckpointManager` in `workspace/verifier/src/checkpoint.rs` captures instantaneous git stash (`git stash create`).
2. **Action Execution**: `PersonaOrchestrator` invokes tools (`Researcher`, `Architect`, `Coder`, `DRC_Reviewer`).
3. **Deterministic Verification**:
   - Firmware: `cargo check --target thumbv7em-none-eabihf` / QEMU test.
   - PCB: `kicad-cli drc --output drc.json`.
   - Web/Perception: `perception_visual_verify` screenshot comparison.
4. **Failure Handling**:
   - If verification fails, stderr is sent back to the `Coder` persona.
   - `OscillationDetector` tracks identical attempts (max threshold = 3).
   - If unrecoverable, instant rollback restores the exact checkpoint SHA.
5. **Success Handling**:
   - If verification passes, `DeltaHarvester` records the diff into `grpo_training_pool` in SurrealDB.
   - `SkillCrystallizer` writes procedural steps to `workspace/skills/`.

---

## 5. JIT MCP Tool Synthesis & Sandbox Flow

```
[Agent Identifies Missing Tool (e.g. specialized SIMD CRC / Netlist Parser / Web Extractor)]
                                │
                                ▼
[JitMcpToolMaker (tool_maker.rs)]
  - Synthesizes standalone Python / Mojo script
  - Writes to isolated /tmp/jit_tools/{tool_name}.py
                                │
                                ▼
[Bubblewrap (bwrap) Isolated Sandbox Test]
  - Unshares all Linux namespaces (--unshare-all)
  - Read-only binds system libraries (/usr, /lib, /lib64)
  - Runs tool self-test (--test)
                                │
                                ▼
[Tool Registration & Dynamic Mounting]
  - If self-test passes, mounts tool into active agent registry
```
