---
okf_version: "0.2"
type: Function
title: safe_step_hook
description: "Safely execute step hook with `catch_unwind`"
resource: targets/llm-pretraining-rs/harness/src/loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T05:14:17Z"
concept_id: targets/llm-pretraining-rs/harness/src/loader/safe_step_hook
language: rust
---

# safe_step_hook

Safely execute step hook with `catch_unwind`

## Signature

```rust
impl SurfaceLoader { pub fn safe_step_hook(
        &self,
        ctx: &StepCtx,
        host: &HostOps,
    ) -> Result<HookActionResult, LoaderError> }
```

## Visibility

- `pub`

## Docstring

Safely execute step hook with `catch_unwind`

## Source
Lines 77–98 in `targets/llm-pretraining-rs/harness/src/loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [loader](/targets/llm-pretraining-rs/harness/src/loader.md) |
