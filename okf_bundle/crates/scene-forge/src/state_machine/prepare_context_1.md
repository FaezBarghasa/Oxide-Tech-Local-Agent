---
okf_version: "0.2"
type: Function
title: prepare_context
description: Prepares and auto-synchronizes the Blender context BEFORE executing an operation.
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/prepare_context_1
language: rust
---

# prepare_context

Prepares and auto-synchronizes the Blender context BEFORE executing an operation.

## Signature

```rust
pub fn prepare_context(
        &mut self,
        required_mode: InteractionMode,
        target_obj: Uuid,
    ) -> Vec<BlenderCommand>
```

## Visibility

- `pub`

## Docstring

Prepares and auto-synchronizes the Blender context BEFORE executing an operation.
Returns a list of auto-injected synchronization commands.

## Source
Lines 90–110 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
