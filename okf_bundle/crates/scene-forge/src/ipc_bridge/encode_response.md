---
okf_version: "0.2"
type: Function
title: encode_response
description: Helper to encode a response into length-prefixed bytes (for server/mock implementation).
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/encode_response
language: rust
---

# encode_response

Helper to encode a response into length-prefixed bytes (for server/mock implementation).

## Signature

```rust
impl BlenderBridge<S> { pub fn encode_response(resp: &BlenderResponse) -> Result<Vec<u8>, SceneForgeError> }
```

## Type Parameters

- `S`

## Visibility

- `pub`

## Docstring

Helper to encode a response into length-prefixed bytes (for server/mock implementation).

## Source
Lines 213–221 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
