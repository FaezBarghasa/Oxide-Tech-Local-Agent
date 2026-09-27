---
okf_version: "0.2"
type: Function
title: classify_action
resource: crates/crucible/src/simulator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/crucible/src/simulator/classify_action
language: rust
---

# classify_action

## Signature

```rust
pub fn classify_action(action: &str) -> ActionKind
```

## Visibility

- `pub`

## Source
Lines 65–104 in `crates/crucible/src/simulator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/crucible/src/simulator.md) |
| called_by | [score_candidate_action](/crates/crucible/src/simulator/score_candidate_action.md) |
