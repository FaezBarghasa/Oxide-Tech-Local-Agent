---
okf_version: "0.2"
type: Class
title: SceneObject
description: Representation of an object tracked in the Shadow Scene Graph.
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/SceneObject
language: rust
---

# SceneObject

Representation of an object tracked in the Shadow Scene Graph.

## Signature

```rust
pub struct SceneObject
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Representation of an object tracked in the Shadow Scene Graph.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `id`
- `name`
- `dimensions`
- `mesh`
- `visible`

## Source
Lines 45–51 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
