---
okf_version: "0.2"
type: Function
title: ffi_build_model
description: ── D. FFI EXPORTS ────────────────────────────────────────────────────────────
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/ffi_build_model
language: rust
---

# ffi_build_model

── D. FFI EXPORTS ────────────────────────────────────────────────────────────

## Signature

```rust
fn ffi_build_model(dims: *const Dims, host: *const HostOps) -> ModelHandle
```

## Docstring

── D. FFI EXPORTS ────────────────────────────────────────────────────────────

## Source
Lines 66–71 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| calls | [build_model_intent](/targets/llm-pretraining-rs/surface/src/lib/build_model_intent.md) |
