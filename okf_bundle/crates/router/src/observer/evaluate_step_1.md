---
okf_version: "0.2"
type: Function
title: evaluate_step
description: "Evaluate an agent's proposed action and generated code for hallucinations and circular loops"
resource: crates/router/src/observer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/observer/evaluate_step_1
language: rust
---

# evaluate_step

Evaluate an agent's proposed action and generated code for hallucinations and circular loops

## Signature

```rust
pub fn evaluate_step(
        &mut self,
        step_index: usize,
        action_name: &str,
        generated_code: &str,
        latency_ms: u64,
    ) -> LoopObservationReport
```

## Visibility

- `pub`

## Docstring

Evaluate an agent's proposed action and generated code for hallucinations and circular loops

## Source
Lines 47–112 in `crates/router/src/observer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [observer](/crates/router/src/observer.md) |
