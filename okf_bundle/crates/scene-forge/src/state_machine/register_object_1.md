---
okf_version: "0.2"
type: Function
title: register_object
description: Register or track a new object in the shadow graph.
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/register_object_1
language: rust
---

# register_object

Register or track a new object in the shadow graph.

## Signature

```rust
pub fn register_object(
        &mut self,
        id: Uuid,
        name: impl Into<String>,
        dimensions: [f32; 3],
        mesh: Option<MeshData>,
    ) -> &mut SceneObject
```

## Visibility

- `pub`

## Docstring

Register or track a new object in the shadow graph.

## Source
Lines 113–124 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
