---
okf_version: "0.2"
type: Function
title: add_cylinder
resource: crates/cad-forge/src/dsl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/cad-forge/src/dsl/add_cylinder
language: rust
---

# add_cylinder

## Signature

```rust
impl CadBuilder { pub fn add_cylinder(mut self, center: Vec3, radius: f32, height: f32) -> (Self, usize) }
```

## Visibility

- `pub`

## Source
Lines 52–60 in `crates/cad-forge/src/dsl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dsl](/crates/cad-forge/src/dsl.md) |
