---
okf_version: "0.2"
type: Function
title: build_kernel_node
resource: crates/re-forge/src/cuda/graph_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T13:54:12Z"
concept_id: crates/re-forge/src/cuda/graph_bridge/build_kernel_node_1
language: rust
---

# build_kernel_node

## Signature

```rust
pub fn build_kernel_node(kernel: &CudaKernel, analysis: &PtxAnalysis) -> CudaKernelNode
```

## Visibility

- `pub`

## Source
Lines 28–39 in `crates/re-forge/src/cuda/graph_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_bridge](/crates/re-forge/src/cuda/graph_bridge.md) |
