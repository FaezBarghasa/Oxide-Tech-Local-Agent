---
okf_version: "0.2"
type: Function
title: pin_current_thread_to_core
description: Pin the calling thread to a specific CPU core ID on Linux.
resource: crates/oxide-core/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/topology/pin_current_thread_to_core
language: rust
---

# pin_current_thread_to_core

Pin the calling thread to a specific CPU core ID on Linux.

## Signature

```rust
impl RuntimeTopology { pub fn pin_current_thread_to_core(core_id: usize) -> bool }
```

## Visibility

- `pub`

## Docstring

Pin the calling thread to a specific CPU core ID on Linux.
[cfg(target_os = "linux")]

## Source
Lines 45–54 in `crates/oxide-core/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-core/src/topology.md) |
