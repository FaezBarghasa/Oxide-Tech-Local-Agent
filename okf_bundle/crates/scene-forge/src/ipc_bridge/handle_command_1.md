---
okf_version: "0.2"
type: Function
title: handle_command
description: Process a command and return the corresponding response.
resource: crates/scene-forge/src/ipc_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:20:16Z"
concept_id: crates/scene-forge/src/ipc_bridge/handle_command_1
language: rust
---

# handle_command

Process a command and return the corresponding response.

## Signature

```rust
pub fn handle_command(&mut self, cmd: BlenderCommand) -> BlenderResponse
```

## Visibility

- `pub`

## Docstring

Process a command and return the corresponding response.

## Source
Lines 88–149 in `crates/scene-forge/src/ipc_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc_bridge](/crates/scene-forge/src/ipc_bridge.md) |
| calls | [cube](/crates/scene-forge/src/mesh/cube.md) |
| calls | [cylinder](/crates/scene-forge/src/mesh/cylinder.md) |
| calls | [sci_fi_crate](/crates/scene-forge/src/mesh/sci_fi_crate.md) |
| calls | [Mesh](/crates/cad-forge/src/kernel/Mesh.md) |
