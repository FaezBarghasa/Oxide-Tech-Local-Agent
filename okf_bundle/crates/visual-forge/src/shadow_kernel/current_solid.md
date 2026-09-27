---
okf_version: "0.2"
type: Function
title: current_solid
description: "Returns the currently active solid representation, if any."
resource: crates/visual-forge/src/shadow_kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/shadow_kernel/current_solid
language: rust
---

# current_solid

Returns the currently active solid representation, if any.

## Signature

```rust
impl ShadowKernel { pub fn current_solid(&self) -> Option<&SolidRepresentation> }
```

## Visibility

- `pub`

## Docstring

Returns the currently active solid representation, if any.

## Source
Lines 71–73 in `crates/visual-forge/src/shadow_kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_kernel](/crates/visual-forge/src/shadow_kernel.md) |
