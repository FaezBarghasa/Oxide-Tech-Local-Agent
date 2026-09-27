---
okf_version: "0.2"
type: Module
title: lib
description: "# Autoresearch Mutable Surface (`surface.rs`)"
resource: targets/llm-pretraining-rs/surface/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/surface/src/lib
language: rust
---

# lib

# Autoresearch Mutable Surface (`surface.rs`)

## Docstring

# Autoresearch Mutable Surface (`surface.rs`)

Budget: <= 420 lines. This IS the pure-Rust rewritten `train.py`.
Uses safe newtypes over the frozen `repr(C)` ABI with zero runtime overhead.

## Relationships

| Type | Target |
|------|--------|
| related | [build_model_intent](/targets/llm-pretraining-rs/surface/src/lib/build_model_intent.md) |
| related | [build_optimizer_intent](/targets/llm-pretraining-rs/surface/src/lib/build_optimizer_intent.md) |
| related | [step_hook_intent](/targets/llm-pretraining-rs/surface/src/lib/step_hook_intent.md) |
| related | [ffi_build_model](/targets/llm-pretraining-rs/surface/src/lib/ffi_build_model.md) |
| related | [ffi_build_optimizer](/targets/llm-pretraining-rs/surface/src/lib/ffi_build_optimizer.md) |
| related | [ffi_step_hook](/targets/llm-pretraining-rs/surface/src/lib/ffi_step_hook.md) |
| related | [ffi_destroy_model](/targets/llm-pretraining-rs/surface/src/lib/ffi_destroy_model.md) |
| related | [oxide_surface_vtable](/targets/llm-pretraining-rs/surface/src/lib/oxide_surface_vtable.md) |
