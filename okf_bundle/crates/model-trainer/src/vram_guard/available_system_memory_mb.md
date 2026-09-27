---
okf_version: "0.2"
type: Function
title: available_system_memory_mb
description: Read available memory on the host system in Megabytes.
resource: crates/model-trainer/src/vram_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/vram_guard/available_system_memory_mb
language: rust
---

# available_system_memory_mb

Read available memory on the host system in Megabytes.

## Signature

```rust
impl VramGuard { pub fn available_system_memory_mb() -> u64 }
```

## Visibility

- `pub`

## Docstring

Read available memory on the host system in Megabytes.

## Source
Lines 49–53 in `crates/model-trainer/src/vram_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vram_guard](/crates/model-trainer/src/vram_guard.md) |
