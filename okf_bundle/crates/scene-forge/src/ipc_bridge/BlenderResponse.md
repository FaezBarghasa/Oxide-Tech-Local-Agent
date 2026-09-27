---
okf_version: "0.2"
type: Class
title: BlenderResponse
description: Responses returned from the 3D host across the binary IPC stream.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/BlenderResponse
language: rust
---

# BlenderResponse

Responses returned from the 3D host across the binary IPC stream.

## Signature

```rust
pub enum BlenderResponse
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Responses returned from the 3D host across the binary IPC stream.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `timestamp_epoch_ms`
- `status_flags`

## Source
Lines 59–68 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
