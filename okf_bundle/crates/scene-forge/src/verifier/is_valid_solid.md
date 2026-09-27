---
okf_version: "0.2"
type: Function
title: is_valid_solid
description: "Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g)."
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/is_valid_solid
language: rust
---

# is_valid_solid

Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g).

## Signature

```rust
impl ManifoldReport { pub fn is_valid_solid(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g).

## Source
Lines 22–26 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
