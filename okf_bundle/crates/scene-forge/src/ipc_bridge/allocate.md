---
okf_version: "0.2"
type: Function
title: allocate
description: "Create or open named shared memory segment in `/dev/shm`"
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/allocate
language: rust
---

# allocate

Create or open named shared memory segment in `/dev/shm`

## Signature

```rust
impl ShmGeometryBuffer { pub fn allocate(name: &str, size_bytes: usize) -> Result<Self, SceneForgeError> }
```

## Visibility

- `pub`

## Docstring

Create or open named shared memory segment in `/dev/shm`

## Source
Lines 261–267 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
