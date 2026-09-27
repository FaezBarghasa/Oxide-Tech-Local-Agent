---
okf_version: "0.2"
type: Function
title: max_shared_memory_bytes
description: Maximum shared memory per SM in bytes
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/max_shared_memory_bytes_1
language: rust
---

# max_shared_memory_bytes

Maximum shared memory per SM in bytes

## Signature

```rust
pub fn max_shared_memory_bytes(&self) -> usize
```

## Visibility

- `pub`

## Docstring

Maximum shared memory per SM in bytes

## Source
Lines 32–41 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
