---
okf_version: "0.2"
type: Function
title: allocate
description: Allocate an anonymous RAM-backed sealed memory descriptor.
resource: crates/oxide-core/src/shm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:03:27Z"
concept_id: crates/oxide-core/src/shm/allocate_1
language: rust
---

# allocate

Allocate an anonymous RAM-backed sealed memory descriptor.

## Signature

```rust
pub fn allocate(name: &str, size_bytes: usize) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Docstring

Allocate an anonymous RAM-backed sealed memory descriptor.

## Source
Lines 13–70 in `crates/oxide-core/src/shm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shm](/crates/oxide-core/src/shm.md) |
