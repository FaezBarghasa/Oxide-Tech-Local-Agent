---
okf_version: "0.2"
type: Function
title: execute
description: Dispatch a command over the binary stream and read the response.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/execute
language: rust
---

# execute

Dispatch a command over the binary stream and read the response.

## Signature

```rust
impl BlenderBridge<S> { pub fn execute(
        &mut self,
        cmd: BlenderCommand,
    ) -> Result<BlenderResponse, SceneForgeError> }
```

## Type Parameters

- `S`

## Visibility

- `pub`

## Docstring

Dispatch a command over the binary stream and read the response.

## Source
Lines 166–210 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
