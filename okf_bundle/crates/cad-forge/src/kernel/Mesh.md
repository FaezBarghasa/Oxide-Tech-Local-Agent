---
okf_version: "0.2"
type: Class
title: Mesh
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/cad-forge/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/kernel/Mesh
language: rust
---

# Mesh

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Mesh
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `vertices`
- `triangles`

## Source
Lines 52–55 in `crates/cad-forge/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/cad-forge/src/kernel.md) |
| called_by | [handle_command](/crates/scene-forge/src/ipc_bridge/handle_command.md) |
