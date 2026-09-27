---
okf_version: "0.2"
type: Class
title: PtxAnalysis
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
concept_id: crates/re-forge/src/cuda/ptx_parser/PtxAnalysis
language: rust
---

# PtxAnalysis

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct PtxAnalysis
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `kernel_name`
- `target_arch`
- `tensor_core_patterns`
- `memory_pattern`
- `inferred_operation`

## Source
Lines 20–26 in `crates/re-forge/src/cuda/ptx_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ptx_parser](/crates/re-forge/src/cuda/ptx_parser.md) |
