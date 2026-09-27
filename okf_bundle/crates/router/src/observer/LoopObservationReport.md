---
okf_version: "0.2"
type: Class
title: LoopObservationReport
description: Observation report evaluating a single step of an execution loop
resource: crates/router/src/observer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/observer/LoopObservationReport
language: rust
---

# LoopObservationReport

Observation report evaluating a single step of an execution loop

## Signature

```rust
pub struct LoopObservationReport
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Observation report evaluating a single step of an execution loop
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `step_index`
- `latency_ms`
- `is_redundant`
- `hallucination_flags`
- `step_efficiency_score`
- `recommendation`

## Source
Lines 6–13 in `crates/router/src/observer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [observer](/crates/router/src/observer.md) |
