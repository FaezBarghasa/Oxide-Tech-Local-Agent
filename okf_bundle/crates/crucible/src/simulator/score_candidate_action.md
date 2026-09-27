---
okf_version: "0.2"
type: Function
title: score_candidate_action
resource: crates/crucible/src/simulator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/crucible/src/simulator/score_candidate_action
language: rust
---

# score_candidate_action

## Signature

```rust
pub fn score_candidate_action(action: &str) -> f64
```

## Visibility

- `pub`

## Source
Lines 106–114 in `crates/crucible/src/simulator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/crucible/src/simulator.md) |
| calls | [classify_action](/crates/crucible/src/simulator/classify_action.md) |
| called_by | [simulate_best_action](/crates/crucible/src/simulator/simulate_best_action.md) |
