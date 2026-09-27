---
okf_version: "0.2"
type: Function
title: new_with_cascade
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/new_with_cascade
language: rust
---

# new_with_cascade

## Signature

```rust
impl DecisionEngine { pub fn new_with_cascade(
        device: DecisionDevice,
        max_batch_size: usize,
        timeout_ms: u64,
        default_cascade: SpeculativeCascadeConfig,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 71–92 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
| calls | [run_worker_loop](/crates/oxide-engines/src/decision_engine/run_worker_loop.md) |
