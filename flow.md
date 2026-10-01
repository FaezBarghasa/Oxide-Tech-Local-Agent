# Oxide-Tech Local Agent OS: System Control Flow (v3.2)

This document details the step-by-step logic, runtime control loops, and execution algorithms that govern Oxide-Tech Local Agent OS.

---

## 1. Universal AI Gateway Routing & Scoring Flow

```
[Inbound Prompt / Chat Request Received at :8080]
                       │
                       ▼
[UniversalRouter (universal_router.rs)]
  - Evaluates Request Strategy (1 of 19 Strategies)
                       │
  ┌────────────────────┼─────────────────────────────────┐
  ▼                    ▼                                 ▼
[Context-Relay Flow]  [Auto 16-Factor Scorer]    [Local-First / Cost-Optimized]
- Inspects session     - Context Proximity        - Evaluates local llama-server
  token usage (85%+)   - Thinking capability      - Zero WAN / Zero token cost
- Hand off if needed   - 429 consecutive fails    - Direct -ngl 99 GPU pipeline
  │                    │                                 │
  └────────────────────┼─────────────────────────────────┘
                       │
                       ▼
[Target Upstream Dispatched]
  - Native llama-server (:8081) / Ollama (:11434) / Cloud Provider
  - Token bucket accounting & sliding reset-window decay recorded
```

---

## 2. Tri-Engine Perception & External Research Flow

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

## 3. Multi-Modal Graph Traversal & Context Pruning Flow

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

## 5. Cyclic FSM & Self-Correction Routing Flow

```
[Task Initiated] ───────► [FSM: Pending]
                               │
                               ▼
                       [FSM: Executing] ◄─────────────────────────┐
                               │                                  │
                 ┌─────────────┴─────────────┐                    │
                 ▼                           ▼                    │
          [Success / Done]           [Error / Compile Fail]       │
                 │                           │                    │
                 ▼                           ▼                    │
         [FSM: Reviewing]           [Retry Count < Max?]          │
                 │                     ├────────► Yes ────────────┘
                 │                     ▼ No
                 │              [FSM: Diagnosing]
                 │                     │
                 │              (Root Cause Analysis)
                 │                     │
                 ▼                     ▼
          [FSM: Completed]      [FSM: NeedsHumanApproval / Escalated]
```

---

## 6. Human-in-the-Loop (HITL) Inbox Protocol Flow

1. **Risk Assessment**: `AgentMode` evaluates tool invocation risk level (`Exec`, `FsWrite`, `NetworkWrite`, `Consequential`).
2. **Approval Suspension**:
   - If action exceeds permission boundary, an `InboxEntry` is placed in `HitlInboxManager`.
   - The executing Tokio task awaits the dedicated `oneshot::Receiver<bool>`.
   - System logs event `JournalEvent::HumanInterruptRequested` in `AgentJournal`.
3. **Operator Resolution**:
   - Operator reviews the command payload and contextual diff via Oxide Agent Studio.
   - Operator responds with `Approved` or `Rejected` (with optional corrective guidance).
4. **Execution Continuation**:
   - `resolve_entry(id, decision)` completes the oneshot channel.
   - Task resumes execution seamlessly or shifts to FSM rollback/diagnostic routing.

---

## 7. JIT MCP Tool Synthesis & Sandbox Flow

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

---

## 8. Polyglot to Rust Refactoring Flow (`forge-rust`)

```
[Foreign Source Code (C/C++, Python, TypeScript, Go, Java, Generic)]
                                │
                                ▼
[LanguageDetector (detector.rs)]
  - Heuristic classifier (extensions, shebangs, syntax signatures)
                                │
                                ▼
[LanguageLifter (src/lifter/*)]
  - Translates foreign AST / regex tokens into Polyglot UIR
                                │
                                ▼
[RefactorPipeline (src/refactor/*)]
  - NamingPass: Enforces snake_case / PascalCase / SCREAMING_SNAKE_CASE
  - OwnershipPass: Pointers & GC refs -> &, &mut, Box<T>, Arc<Mutex<T>>
  - ErrorHandlingPass: -1/errno/exceptions -> Result<T, E> & Option<T>
  - CompositionPass: Methods & classes -> struct impl blocks & traits
  - ConcurrencyPass: Async / goroutines -> Tokio runtime integration
                                │
                                ▼
[RustEmitter (emitter.rs)]
  - Generates idiomatic Rust source code with formatted derives & imports
                                │
                                ▼
[RustVerifier (verifier.rs) & ProjectScaffolder (scaffold.rs)]
  - Validates syntax with syn::parse_file
  - Generates Cargo.toml, src/lib.rs / src/main.rs, and README.md
```
