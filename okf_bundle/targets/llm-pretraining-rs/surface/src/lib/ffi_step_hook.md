---
okf_version: "0.2"
type: Function
title: ffi_step_hook
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/ffi_step_hook
language: rust
---

# ffi_step_hook

## Signature

```rust
fn ffi_step_hook(
    raw_ctx: *const StepCtx,
    host: *const HostOps,
) -> surface_api::abi::HookActionResult
```

## Source
Lines 83–93 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| calls | [step_hook_intent](/targets/llm-pretraining-rs/surface/src/lib/step_hook_intent.md) |
