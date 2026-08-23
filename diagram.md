# Oxide-Tech Local Agent OS: System Diagrams

This document contains Mermaid diagrams illustrating request handling, graph traversal, multi-agent loop orchestration, and self-evolution cycles.

---

## 1. End-to-End System Execution Sequence

```mermaid
sequenceDiagram
    autonumber
    actor User as User / IDE / Studio UI
    participant GW as Dual-Protocol Gateway (:8080)
    participant Graph as Graph Engineering Engine
    participant SGLang as SGLang TP=2 (:30000)
    participant Verifier as Deterministic Verifiers
    participant Evolver as Self-Evolution Engine
    participant DB as SurrealDB v3 / Qdrant

    User->>GW: Submit Goal (e.g. "Build SPI DMA Driver in no_std")
    GW->>Graph: Query AST & Downstream Impact Surface
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

## 2. Multi-Agent Persona Handoff & Oscillation Guard

```mermaid
stateDiagram-v2
    [*] --> ArchitectPlanning : User Goal Received
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

## 3. Tri-Fold Self-Evolution Engine Flow

```mermaid
graph LR
    subgraph SelfImprovement [1. Self-Improvement]
        Fix[Compiler Fix Pair] --> Delta[DeltaHarvester]
        Delta --> Training[SurrealDB grpo_training_pool]
    end

    subgraph SelfSkillMaking [2. Self-Skill Making]
        Traj[Multi-Turn Trajectory] --> Crystallizer[SkillOpt Crystallizer]
        Crystallizer --> SkillMD[workspace/skills/domain/skill.md]
    end

    subgraph SelfToolMaking [3. Self-Tool Making]
        Missing[Missing CLI / SIMD Utility] --> ToolMaker[JIT MCP Tool Maker]
        ToolMaker --> Bwrap[Bubblewrap bwrap Sandbox Test]
        Bwrap --> Mounted[Mount to Agent Tool Registry]
    end
```
