# Oxide-Tech Local Agent OS: System Diagrams (v2.1)

This document contains Mermaid diagrams illustrating request handling, tri-engine perception dispatch, graph traversal, multi-agent loop orchestration, and self-evolution cycles.

---

## 1. End-to-End System Execution Sequence with Perception Layer

```mermaid
sequenceDiagram
    autonumber
    actor User as User / IDE / Studio UI
    participant GW as Dual-Protocol Gateway (:8080)
    participant Percept as Perception Router (Scrapling/PinchTab/Kitesurf)
    participant Graph as Graph Engineering Engine
    participant SGLang as SGLang TP=2 (:30000)
    participant Verifier as Deterministic Verifiers
    participant Evolver as Self-Evolution Engine
    participant DB as SurrealDB v3 / Qdrant

    User->>GW: Submit Goal (e.g. "Build SPI DMA Driver in no_std using new crate")
    GW->>Percept: Dispatch Research Query (docs.rs / GitHub / Web)
    alt Fast Local Scrape
        Percept->>Percept: Tier 1: Scrapling (sub-200ms)
    else Interactive / Form / Protected
        Percept->>Percept: Tier 2: PinchTab Local Daemon (:9876)
    else Cloud V8 Parallel / Turnstile Challenge
        Percept->>Percept: Tier 3: Cloudflare Kitesurf Isolates
    end
    Percept->>DB: Ingest Grounded Docs with Provenance (URL, engine, timestamp)
    Percept-->>GW: Grounded Research Findings

    GW->>Graph: Map External Dependency Node & Query AST Impact
    Graph->>DB: Fetch 1-hop & 2-hop topological callers/callees
    Graph-->>GW: Return Compact Pruned Context (-80% tokens)

    GW->>SGLang: Route to Dynamic LoRA Adapter (lora_embedded_rust_v2)
    SGLang-->>GW: Generate Structured Plan & Code

    GW->>Verifier: Checkpoint tree (git stash create) & Run cargo check / QEMU
    alt Verification Fails (Compiler Error)
        Verifier-->>GW: Error diagnostics (stderr)
        GW->>SGLang: Feed error to Verifier / Coder Persona for self-correction
        SGLang-->>GW: Generate fixed code
        GW->>Verifier: Re-run verification (Max 5 attempts)
    end

    Verifier-->>GW: Verification PASS
    GW->>Evolver: DeltaHarvester logs fix -> grpo_training_pool
    Evolver->>DB: Store verified trajectory & update SKILL.md
    GW-->>User: 200 OK Execution Result + Diff Summary
```

---

## 2. Multi-Agent Persona Handoff (Researcher -> Architect -> Coder -> DRC)

```mermaid
stateDiagram-v2
    [*] --> PerceptionResearch : User Goal Received
    PerceptionResearch --> ArchitectPlanning : Grounded Research Context
    ArchitectPlanning --> CoderSynthesis : Decompose Plan into Step DAG
    CoderSynthesis --> VerifierAudit : Emit Code / Schematic

    state VerifierAudit {
        [*] --> CompileValidation
        CompileValidation --> DRCValidation
        DRCValidation --> UnitTesting
    }

    VerifierAudit --> ObserverReview : Verification Result
    
    state ObserverReview {
        [*] --> CheckOscillation
        CheckOscillation --> LoopScoring : Loop Attempts < 3
        CheckOscillation --> ForceHalt : Loop Attempts >= 3 (Oscillation Guard)
    }

    ObserverReview --> CoderSynthesis : Non-zero exit code (Self-Correct)
    ObserverReview --> SkillCrystallization : All Tests Passed
    SkillCrystallization --> [*] : Update workspace/skills/
```

---

## 3. Tri-Engine Perception Dispatch Hierarchy

```mermaid
graph TD
    Query[Target URL / Technical Query] --> Dispatcher[Perception Router]

    Dispatcher -->|Static / Docs / GitHub| T1[Tier 1: d4vinci/Scrapling (Local Fast)]
    Dispatcher -->|Interactive / Auth / Local UI| T2[Tier 2: pinchtab/pinchtab (Local Daemon)]
    Dispatcher -->|JS-Heavy / Protected / Visual DRC| T3[Tier 3: Cloudflare Kitesurf (Cloud V8)]

    T1 -->|HTTP 403 / Cloudflare Challenge| T2
    T2 -->|Massive Parallel / Offload| T3

    T1 --> Grounding[Provenance Metadata Tagging]
    T2 --> Grounding
    T3 --> Grounding

    Grounding --> Qdrant[Qdrant Hybrid Vector Store]
    Grounding --> Surreal[SurrealDB v3 Research Cache]
```

---

## 4. Tri-Fold Self-Evolution Engine Flow

```mermaid
graph LR
    subgraph SelfImprovement [1. Self-Improvement]
        Fix[Compiler Fix Pair] --> Delta[DeltaHarvester]
        Delta --> Training[SurrealDB grpo_training_pool]
    end

    subgraph SelfSkillMaking [2. Self-Skill Making]
        Traj[Multi-Turn Research + Execution Trajectory] --> Crystallizer[SkillOpt Crystallizer]
        Crystallizer --> SkillMD[workspace/skills/domain/skill.md]
    end

    subgraph SelfToolMaking [3. Self-Tool Making]
        Missing[Missing CLI / SIMD Utility] --> ToolMaker[JIT MCP Tool Maker]
        ToolMaker --> Bwrap[Bubblewrap bwrap Sandbox Test]
        Bwrap --> Mounted[Mount to Agent Tool Registry]
    end
```
