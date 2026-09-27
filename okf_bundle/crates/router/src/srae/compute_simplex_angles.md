---
okf_version: "0.2"
type: Function
title: compute_simplex_angles
description: Compute the SRAE angles representing regular simplex vertices
resource: crates/router/src/srae.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/srae/compute_simplex_angles
language: rust
---

# compute_simplex_angles

Compute the SRAE angles representing regular simplex vertices

## Signature

```rust
impl SimplexRotaryEncoding { pub fn compute_simplex_angles(&self) -> Array2<f32> }
```

## Visibility

- `pub`

## Docstring

Compute the SRAE angles representing regular simplex vertices

## Source
Lines 18–34 in `crates/router/src/srae.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [srae](/crates/router/src/srae.md) |
