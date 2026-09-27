---
okf_version: "0.2"
type: Function
title: build_model_intent
description: ── A. MODEL INTENT ──────────────────────────────────────────────────────────
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/build_model_intent
language: rust
---

# build_model_intent

── A. MODEL INTENT ──────────────────────────────────────────────────────────

## Signature

```rust
pub fn build_model_intent(d: &Dims, h: &'h HostOps) -> Model<'h>
```

## Type Parameters

- `'h`

## Visibility

- `pub`

## Docstring

── A. MODEL INTENT ──────────────────────────────────────────────────────────

## Source
Lines 10–34 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| called_by | [ffi_build_model](/targets/llm-pretraining-rs/surface/src/lib/ffi_build_model.md) |
