---
okf_version: "0.2"
type: Function
title: encode_command
description: Helper to encode a command into length-prefixed bytes.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/encode_command
language: rust
---

# encode_command

Helper to encode a command into length-prefixed bytes.

## Signature

```rust
impl BlenderBridge<S> { pub fn encode_command(cmd: &BlenderCommand) -> Result<Vec<u8>, SceneForgeError> }
```

## Type Parameters

- `S`

## Visibility

- `pub`

## Docstring

Helper to encode a command into length-prefixed bytes.

## Source
Lines 224–232 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
