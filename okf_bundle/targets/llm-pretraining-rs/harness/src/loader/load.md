---
okf_version: "0.2"
type: Function
title: load
description: Load a candidate cdylib and verify ABI
resource: targets/llm-pretraining-rs/harness/src/loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T05:14:17Z"
concept_id: targets/llm-pretraining-rs/harness/src/loader/load
language: rust
---

# load

Load a candidate cdylib and verify ABI

## Signature

```rust
impl SurfaceLoader { pub fn load(cdylib_path: &Path) -> Result<Self, LoaderError> }
```

## Visibility

- `pub`

## Docstring

Load a candidate cdylib and verify ABI

## Source
Lines 30–50 in `targets/llm-pretraining-rs/harness/src/loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [loader](/targets/llm-pretraining-rs/harness/src/loader.md) |
