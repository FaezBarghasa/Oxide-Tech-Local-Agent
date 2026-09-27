---
okf_version: "0.2"
type: Function
title: apply_command
description: "Apply state transition resulting from a `BlenderCommand`."
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/apply_command
language: rust
---

# apply_command

Apply state transition resulting from a `BlenderCommand`.

## Signature

```rust
impl ShadowSceneGraph { pub fn apply_command(&mut self, cmd: &BlenderCommand) }
```

## Visibility

- `pub`

## Docstring

Apply state transition resulting from a `BlenderCommand`.

## Source
Lines 137–155 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
