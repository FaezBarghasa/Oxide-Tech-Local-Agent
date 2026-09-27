---
okf_version: "0.2"
type: Function
title: simulate_best_action
resource: crates/crucible/src/simulator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/crucible/src/simulator/simulate_best_action
language: rust
---

# simulate_best_action

## Signature

```rust
impl CrucibleEngine { fn simulate_best_action(
        &self,
        snapshot: &WorkspaceSnapshot,
        candidate_actions: &[String],
    ) -> Option<SimulationEvaluation> }
```

## Source
Lines 34–53 in `crates/crucible/src/simulator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/crucible/src/simulator.md) |
| calls | [score_candidate_action](/crates/crucible/src/simulator/score_candidate_action.md) |
