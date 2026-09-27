---
okf_version: "0.2"
type: Function
title: host_create_adamw
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/host_create_adamw
language: rust
---

# host_create_adamw

## Signature

```rust
fn host_create_adamw(
    _model: ModelHandle,
    _lr: f64,
    _beta1: f64,
    _beta2: f64,
    _fused: bool,
) -> OptimHandle
```

## Source
Lines 51–59 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
| calls | [OptimHandle](/crates/surface-api/src/abi/OptimHandle.md) |
