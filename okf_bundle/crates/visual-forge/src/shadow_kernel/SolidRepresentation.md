---
okf_version: "0.2"
type: Class
title: SolidRepresentation
description: In-memory representation of an evaluated solid inside the Shadow Kernel sandbox.
resource: crates/visual-forge/src/shadow_kernel.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/shadow_kernel/SolidRepresentation
language: rust
---

# SolidRepresentation

In-memory representation of an evaluated solid inside the Shadow Kernel sandbox.

## Signature

```rust
pub struct SolidRepresentation
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

In-memory representation of an evaluated solid inside the Shadow Kernel sandbox.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `id`
- `name`
- `sdf`
- `bounds`
- `estimated_volume_mm3`

## Source
Lines 34–40 in `crates/visual-forge/src/shadow_kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_kernel](/crates/visual-forge/src/shadow_kernel.md) |
