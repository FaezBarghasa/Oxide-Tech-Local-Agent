---
okf_version: "0.2"
type: Function
title: oxide_decision_engine_create
description: "Allocate and initialize a new `DecisionEngine` on the heap."
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/oxide_decision_engine_create
language: rust
---

# oxide_decision_engine_create

Allocate and initialize a new `DecisionEngine` on the heap.

## Signature

```rust
pub fn oxide_decision_engine_create(
    max_batch_size: u32,
    timeout_ms: u64,
) -> *mut DecisionEngine
```

## Decorators

- `unsafe(no_mangle)`

## Visibility

- `pub`

## Docstring

Allocate and initialize a new `DecisionEngine` on the heap.

# Safety
Caller must free using `oxide_decision_engine_free`.
[unsafe(no_mangle)]

## Source
Lines 70–76 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
| called_by | [test_c_ffi_json_bridge](/crates/oxide-engines/src/mobile/test_c_ffi_json_bridge.md) |
