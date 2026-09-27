---
okf_version: "0.2"
type: Function
title: step_hook_intent
description: ── C. STEP HOOK (Budgeted <= 60 lines) ───────────────────────────────────────
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib/step_hook_intent
language: rust
---

# step_hook_intent

── C. STEP HOOK (Budgeted <= 60 lines) ───────────────────────────────────────

## Signature

```rust
pub fn step_hook_intent(ctx: &mut StepContext<'_>, _h: &Host<'_>) -> HookAction
```

## Visibility

- `pub`

## Docstring

── C. STEP HOOK (Budgeted <= 60 lines) ───────────────────────────────────────

## Source
Lines 51–62 in `targets/llm-pretraining-rs/surface/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/surface/src/lib.md) |
| called_by | [ffi_step_hook](/targets/llm-pretraining-rs/surface/src/lib/ffi_step_hook.md) |
