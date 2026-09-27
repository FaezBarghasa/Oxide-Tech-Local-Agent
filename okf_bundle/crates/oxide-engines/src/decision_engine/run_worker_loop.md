---
okf_version: "0.2"
type: Function
title: run_worker_loop
description: "--- Dedicated Worker Loop with Dynamic Micro-Batching ---"
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/run_worker_loop
language: rust
---

# run_worker_loop

--- Dedicated Worker Loop with Dynamic Micro-Batching ---

## Signature

```rust
fn run_worker_loop(
    rx: flume::Receiver<InferenceJob>,
    device: DecisionDevice,
    max_batch_size: usize,
    max_wait: Duration,
)
```

## Docstring

--- Dedicated Worker Loop with Dynamic Micro-Batching ---

## Source
Lines 124–159 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
| calls | [process_batch](/crates/oxide-engines/src/decision_engine/process_batch.md) |
| called_by | [new_with_cascade](/crates/oxide-engines/src/decision_engine/new_with_cascade.md) |
