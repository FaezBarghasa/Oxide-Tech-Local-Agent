---
okf_version: "0.2"
type: Class
title: SimulationEvaluation
description: "[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]"
resource: crates/crucible/src/simulator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/crucible/src/simulator/SimulationEvaluation
language: rust
---

# SimulationEvaluation

[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Signature

```rust
pub struct SimulationEvaluation
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `chosen_action`
- `projected_score`
- `branch_depth`
- `latency_us`

## Source
Lines 5–10 in `crates/crucible/src/simulator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/crucible/src/simulator.md) |
