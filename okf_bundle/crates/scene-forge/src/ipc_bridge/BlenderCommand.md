---
okf_version: "0.2"
type: Class
title: BlenderCommand
description: Commands dispatched across the Postcard binary IPC bridge to the 3D host/addon.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/BlenderCommand
language: rust
---

# BlenderCommand

Commands dispatched across the Postcard binary IPC bridge to the 3D host/addon.

## Signature

```rust
pub enum BlenderCommand
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Commands dispatched across the Postcard binary IPC bridge to the 3D host/addon.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `primitive_type`
- `dimensions`
- `axis`
- `distance`
- `offset`
- `segments`
- `target_a`
- `target_b`
- `target_a`
- `target_b`
- `target`
- `translation`
- `rotation`
- `scale`
- `timestamp_epoch_ms`

## Source
Lines 24–55 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
