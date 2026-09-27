---
okf_version: "0.2"
type: Class
title: BridgeSupervisor
description: Bidirectional heartbeat supervisor for the Python subprocess bridge
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/BridgeSupervisor
language: rust
---

# BridgeSupervisor

Bidirectional heartbeat supervisor for the Python subprocess bridge

## Signature

```rust
pub struct BridgeSupervisor
```

## Visibility

- `pub`

## Docstring

Bidirectional heartbeat supervisor for the Python subprocess bridge

## Methods

- `child_pid`
- `heartbeat_interval_ms`
- `consecutive_failures`
- `max_failures`

## Source
Lines 276–281 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
