---
okf_version: "0.2"
type: Class
title: StepInvariantMetrics
description: Step-level invariant execution metrics for granular agent evaluation
resource: crates/benchmark-harness/src/harness.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:benchmark-harness"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:43:50Z"
concept_id: crates/benchmark-harness/src/harness/StepInvariantMetrics
language: rust
---

# StepInvariantMetrics

Step-level invariant execution metrics for granular agent evaluation

## Signature

```rust
pub struct StepInvariantMetrics
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, Default)`

## Visibility

- `pub`

## Docstring

Step-level invariant execution metrics for granular agent evaluation
[derive(Debug, Clone, Serialize, Deserialize, Default)]

## Methods

- `tool_selection_accuracy`
- `schema_validity`
- `recovery_efficiency`
- `cost_per_success`

## Source
Lines 7–16 in `crates/benchmark-harness/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/benchmark-harness/src/harness.md) |
