---
okf_version: "0.2"
type: Function
title: from_compute_capability
description: "Detect architecture from device compute capability (major, minor)"
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/from_compute_capability_1
language: rust
---

# from_compute_capability

Detect architecture from device compute capability (major, minor)

## Signature

```rust
pub fn from_compute_capability(major: u32, minor: u32) -> Self
```

## Visibility

- `pub`

## Docstring

Detect architecture from device compute capability (major, minor)

## Source
Lines 19–29 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
