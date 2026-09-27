---
okf_version: "0.2"
type: Function
title: record_heartbeat_failure
description: Record a failed heartbeat and check if process must be terminated and respawned
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/record_heartbeat_failure
language: rust
---

# record_heartbeat_failure

Record a failed heartbeat and check if process must be terminated and respawned

## Signature

```rust
impl BridgeSupervisor { pub fn record_heartbeat_failure(&mut self) -> bool }
```

## Visibility

- `pub`

## Docstring

Record a failed heartbeat and check if process must be terminated and respawned

## Source
Lines 294–297 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
