---
okf_version: "0.2"
type: Function
title: terminate_hung_process
description: Terminate unresponsive child process via SIGKILL
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/terminate_hung_process
language: rust
---

# terminate_hung_process

Terminate unresponsive child process via SIGKILL

## Signature

```rust
impl BridgeSupervisor { pub fn terminate_hung_process(&self) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Terminate unresponsive child process via SIGKILL
[cfg(target_os = "linux")]

## Source
Lines 306–313 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
