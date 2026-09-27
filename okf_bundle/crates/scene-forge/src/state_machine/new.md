---
okf_version: "0.2"
type: Function
title: new
resource: crates/scene-forge/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/state_machine/new
language: rust
---

# new

## Signature

```rust
impl SceneObject { pub fn new(id: Uuid, name: impl Into<String>, dimensions: [f32; 3]) -> Self }
```

## Visibility

- `pub`

## Source
Lines 54–62 in `crates/scene-forge/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/scene-forge/src/state_machine.md) |
