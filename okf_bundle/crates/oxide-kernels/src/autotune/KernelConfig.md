---
okf_version: "0.2"
type: Class
title: KernelConfig
description: "Fused Kernel Tile & Pipeline Configuration"
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/KernelConfig
language: rust
---

# KernelConfig

Fused Kernel Tile & Pipeline Configuration

## Signature

```rust
pub struct KernelConfig
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Fused Kernel Tile & Pipeline Configuration
[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `block_m`
- `block_n`
- `block_k`
- `num_warps`
- `num_stages`
- `use_tma`
- `use_fp8`

## Source
Lines 65–73 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
