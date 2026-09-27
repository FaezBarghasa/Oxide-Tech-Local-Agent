---
okf_version: "0.2"
type: Function
title: extract_kernels
description: Extracts PTX and SASS from a CUDA fatbin or shared library using cuobjdump
resource: crates/re-forge/src/cuda/analyzer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/re-forge/src/cuda/analyzer/extract_kernels_1
language: rust
---

# extract_kernels

Extracts PTX and SASS from a CUDA fatbin or shared library using cuobjdump

## Signature

```rust
pub fn extract_kernels(&self, binary_bytes: &[u8]) -> Result<Vec<CudaKernel>>
```

## Visibility

- `pub`

## Docstring

Extracts PTX and SASS from a CUDA fatbin or shared library using cuobjdump

## Source
Lines 33–105 in `crates/re-forge/src/cuda/analyzer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analyzer](/crates/re-forge/src/cuda/analyzer.md) |
