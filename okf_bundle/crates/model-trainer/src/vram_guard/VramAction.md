---
okf_version: "0.2"
type: Class
title: VramAction
description: Adaptive action prescribed by the VRAM Guard to prevent Out-Of-Memory aborts.
resource: crates/model-trainer/src/vram_guard.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/vram_guard/VramAction
language: rust
---

# VramAction

Adaptive action prescribed by the VRAM Guard to prevent Out-Of-Memory aborts.

## Signature

```rust
pub enum VramAction
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Adaptive action prescribed by the VRAM Guard to prevent Out-Of-Memory aborts.
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `suggested_batch`
- `required_offload_mb`

## Source
Lines 5–16 in `crates/model-trainer/src/vram_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vram_guard](/crates/model-trainer/src/vram_guard.md) |
