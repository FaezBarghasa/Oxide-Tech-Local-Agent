---
okf_version: "0.2"
type: Function
title: terminate_hung_process
description: "[cfg(not(target_os = \"linux\"))]"
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/terminate_hung_process_3
language: rust
---

# terminate_hung_process

[cfg(not(target_os = "linux"))]

## Signature

```rust
pub fn terminate_hung_process(&self) -> Result<(), String>
```

## Decorators

- `cfg(not(target_os = "linux"))`

## Visibility

- `pub`

## Docstring

[cfg(not(target_os = "linux"))]

## Source
Lines 316–318 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
