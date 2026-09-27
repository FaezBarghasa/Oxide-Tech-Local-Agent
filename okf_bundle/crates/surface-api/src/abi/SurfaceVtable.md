---
okf_version: "0.2"
type: Class
title: SurfaceVtable
description: Vtable exported by the surface cdylib
resource: crates/surface-api/src/abi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:surface-api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/surface-api/src/abi/SurfaceVtable
language: rust
---

# SurfaceVtable

Vtable exported by the surface cdylib

## Signature

```rust
pub struct SurfaceVtable
```

## Decorators

- `repr(C)`

## Visibility

- `pub`

## Docstring

Vtable exported by the surface cdylib
[repr(C)]

## Methods

- `abi_version`
- `build_model`
- `build_optimizer`
- `step_hook`
- `destroy_model`

## Source
Lines 99–105 in `crates/surface-api/src/abi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abi](/crates/surface-api/src/abi.md) |
