---
okf_version: "0.2"
type: Class
title: MemoryAccessPattern
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/re-forge/src/cuda/ptx_parser.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/re-forge/src/cuda/ptx_parser/MemoryAccessPattern
language: rust
---

# MemoryAccessPattern

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct MemoryAccessPattern
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `uses_shared_tiling`
- `shared_memory_bytes`
- `uses_coalesced_global`
- `uses_async_copy`

## Source
Lines 12–17 in `crates/re-forge/src/cuda/ptx_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ptx_parser](/crates/re-forge/src/cuda/ptx_parser.md) |
