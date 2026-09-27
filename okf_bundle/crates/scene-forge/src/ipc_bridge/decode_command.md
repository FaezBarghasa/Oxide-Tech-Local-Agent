---
okf_version: "0.2"
type: Function
title: decode_command
description: Helper to decode a length-prefixed payload buffer.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/decode_command
language: rust
---

# decode_command

Helper to decode a length-prefixed payload buffer.

## Signature

```rust
impl BlenderBridge<S> { pub fn decode_command(bytes: &[u8]) -> Result<BlenderCommand, SceneForgeError> }
```

## Type Parameters

- `S`

## Visibility

- `pub`

## Docstring

Helper to decode a length-prefixed payload buffer.

## Source
Lines 235–249 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
