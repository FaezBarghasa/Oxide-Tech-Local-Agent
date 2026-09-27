---
okf_version: "0.2"
type: Class
title: Body
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/cad-forge/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/kernel/Body
language: rust
---

# Body

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Body
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `volume_mm3`
- `surface_area_mm2`
- `center_of_mass`
- `bounding_box_min`
- `bounding_box_max`

## Source
Lines 42–49 in `crates/cad-forge/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/cad-forge/src/kernel.md) |
