---
okf_version: "0.2"
type: Function
title: apply_group_scale
description: Apply FP16 group scaling factor (g128) to unpacked ternary weights.
resource: crates/oxide-kernels/src/ternary.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-kernels/src/ternary/apply_group_scale_1
language: rust
---

# apply_group_scale

Apply FP16 group scaling factor (g128) to unpacked ternary weights.

## Signature

```rust
pub fn apply_group_scale(&self, weights: &mut [f32], scales: &[f32]) -> Result<(), OxideError>
```

## Visibility

- `pub`

## Docstring

Apply FP16 group scaling factor (g128) to unpacked ternary weights.

## Source
Lines 65–83 in `crates/oxide-kernels/src/ternary.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ternary](/crates/oxide-kernels/src/ternary.md) |
