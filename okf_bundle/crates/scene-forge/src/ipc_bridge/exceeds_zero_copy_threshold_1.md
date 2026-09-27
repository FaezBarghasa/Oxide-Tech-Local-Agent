---
okf_version: "0.2"
type: Function
title: exceeds_zero_copy_threshold
description: Check if payload exceeds zero-copy threshold (2 MB)
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/exceeds_zero_copy_threshold_1
language: rust
---

# exceeds_zero_copy_threshold

Check if payload exceeds zero-copy threshold (2 MB)

## Signature

```rust
pub fn exceeds_zero_copy_threshold(bytes_len: usize) -> bool
```

## Visibility

- `pub`

## Docstring

Check if payload exceeds zero-copy threshold (2 MB)

## Source
Lines 270–272 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
