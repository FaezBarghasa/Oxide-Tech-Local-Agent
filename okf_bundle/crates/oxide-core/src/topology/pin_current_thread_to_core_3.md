---
okf_version: "0.2"
type: Function
title: pin_current_thread_to_core
description: Fallback for non-Linux OS.
resource: crates/oxide-core/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/topology/pin_current_thread_to_core_3
language: rust
---

# pin_current_thread_to_core

Fallback for non-Linux OS.

## Signature

```rust
pub fn pin_current_thread_to_core(_core_id: usize) -> bool
```

## Decorators

- `cfg(not(target_os = "linux"))`

## Visibility

- `pub`

## Docstring

Fallback for non-Linux OS.
[cfg(not(target_os = "linux"))]

## Source
Lines 58–60 in `crates/oxide-core/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-core/src/topology.md) |
