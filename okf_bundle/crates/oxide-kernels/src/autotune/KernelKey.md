---
okf_version: "0.2"
type: Class
title: KernelKey
description: Autotuner Cache Key
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/KernelKey
language: rust
---

# KernelKey

Autotuner Cache Key

## Signature

```rust
pub struct KernelKey
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Autotuner Cache Key
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Methods

- `arch`
- `m`
- `n`
- `k`
- `is_fp8`

## Source
Lines 91–97 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
