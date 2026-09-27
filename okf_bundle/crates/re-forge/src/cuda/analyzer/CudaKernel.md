---
okf_version: "0.2"
type: Class
title: CudaKernel
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/re-forge/src/cuda/analyzer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/re-forge/src/cuda/analyzer/CudaKernel
language: rust
---

# CudaKernel

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct CudaKernel
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `arch`
- `ptx_code`
- `sass_code`
- `registers_used`
- `shared_memory_bytes`

## Source
Lines 14–21 in `crates/re-forge/src/cuda/analyzer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analyzer](/crates/re-forge/src/cuda/analyzer.md) |
