---
okf_version: "0.2"
type: Function
title: ffi_build_optimizer
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/ffi_build_optimizer
language: rust
---

# ffi_build_optimizer

## Signature

```rust
fn ffi_build_optimizer(model: ModelHandle, host: *const HostOps) -> OptimHandle
```

## Source
Lines 73–81 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| calls | [build_optimizer_intent](/targets/llm-pretraining-rs/surface/src/lib/build_optimizer_intent.md) |
