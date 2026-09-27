---
okf_version: "0.2"
type: Class
title: CudaKernelNode
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/re-forge/src/cuda/graph_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T13:54:12Z"
concept_id: crates/re-forge/src/cuda/graph_bridge/CudaKernelNode
language: rust
---

# CudaKernelNode

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct CudaKernelNode
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
- `has_tensor_core`
- `inferred_operation`

## Source
Lines 6–15 in `crates/re-forge/src/cuda/graph_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_bridge](/crates/re-forge/src/cuda/graph_bridge.md) |
