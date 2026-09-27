---
okf_version: "0.2"
type: Function
title: host_create_model
description: ── Default HostOps Implementation ───────────────────────────────────────────
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/host_create_model
language: rust
---

# host_create_model

── Default HostOps Implementation ───────────────────────────────────────────

## Signature

```rust
fn host_create_model(_dims: *const Dims) -> ModelHandle
```

## Docstring

── Default HostOps Implementation ───────────────────────────────────────────

## Source
Lines 16–18 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
| calls | [ModelHandle](/crates/surface-api/src/abi/ModelHandle.md) |
