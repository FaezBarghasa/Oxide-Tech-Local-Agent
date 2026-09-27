---
okf_version: "0.2"
type: Module
title: loader
description: "# Dynamic Hot-Swap Loader with `catch_unwind` Fault Isolation"
resource: targets/llm-pretraining-rs/harness/src/loader.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T05:14:17Z"
concept_id: targets/llm-pretraining-rs/harness/src/loader
language: rust
---

# loader

# Dynamic Hot-Swap Loader with `catch_unwind` Fault Isolation

## Docstring

# Dynamic Hot-Swap Loader with `catch_unwind` Fault Isolation

Loads the candidate `cdylib`, verifies ABI version, and wraps all surface
callbacks in `catch_unwind` to prevent panics from terminating the harness.

## Relationships

| Type | Target |
|------|--------|
| related | [LoaderError](/targets/llm-pretraining-rs/harness/src/loader/LoaderError.md) |
| related | [SurfaceLoader](/targets/llm-pretraining-rs/harness/src/loader/SurfaceLoader.md) |
| related | [load](/targets/llm-pretraining-rs/harness/src/loader/load.md) |
| related | [safe_build_model](/targets/llm-pretraining-rs/harness/src/loader/safe_build_model.md) |
| related | [safe_step_hook](/targets/llm-pretraining-rs/harness/src/loader/safe_step_hook.md) |
| related | [load](/targets/llm-pretraining-rs/harness/src/loader/load.md) |
| related | [safe_build_model](/targets/llm-pretraining-rs/harness/src/loader/safe_build_model.md) |
| related | [safe_step_hook](/targets/llm-pretraining-rs/harness/src/loader/safe_step_hook.md) |
| related | [libloading](/_dependencies/cargo/libloading.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
