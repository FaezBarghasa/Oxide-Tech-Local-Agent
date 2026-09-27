---
okf_version: "0.2"
type: Function
title: get_object_mut
description: Retrieve an object mutably by its UUID.
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/get_object_mut_1
language: rust
---

# get_object_mut

Retrieve an object mutably by its UUID.

## Signature

```rust
pub fn get_object_mut(&mut self, id: Uuid) -> Option<&mut SceneObject>
```

## Visibility

- `pub`

## Docstring

Retrieve an object mutably by its UUID.

## Source
Lines 132–134 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
