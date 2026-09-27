---
okf_version: "0.2"
type: Function
title: safe_build_model
description: "Safely build model with `catch_unwind`"
resource: targets/llm-pretraining-rs/harness/src/loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T05:14:17Z"
concept_id: targets/llm-pretraining-rs/harness/src/loader/safe_build_model_1
language: rust
---

# safe_build_model

Safely build model with `catch_unwind`

## Signature

```rust
pub fn safe_build_model(
        &self,
        dims: &Dims,
        host: &HostOps,
    ) -> Result<ModelHandle, LoaderError>
```

## Visibility

- `pub`

## Docstring

Safely build model with `catch_unwind`

## Source
Lines 53–74 in `targets/llm-pretraining-rs/harness/src/loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [loader](/targets/llm-pretraining-rs/harness/src/loader.md) |
