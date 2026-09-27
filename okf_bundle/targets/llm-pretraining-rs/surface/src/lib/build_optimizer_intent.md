---
okf_version: "0.2"
type: Function
title: build_optimizer_intent
description: ── B. OPTIMIZER INTENT ──────────────────────────────────────────────────────
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/build_optimizer_intent
language: rust
---

# build_optimizer_intent

── B. OPTIMIZER INTENT ──────────────────────────────────────────────────────

## Signature

```rust
pub fn build_optimizer_intent(m: &Model<'h>, h: &'h HostOps) -> Optim<'h>
```

## Type Parameters

- `'h`

## Visibility

- `pub`

## Docstring

── B. OPTIMIZER INTENT ──────────────────────────────────────────────────────

## Source
Lines 38–47 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| called_by | [ffi_build_optimizer](/targets/llm-pretraining-rs/surface/src/lib/ffi_build_optimizer.md) |
