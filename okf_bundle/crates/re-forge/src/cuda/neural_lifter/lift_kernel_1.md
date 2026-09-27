---
okf_version: "0.2"
type: Function
title: lift_kernel
resource: crates/re-forge/src/cuda/neural_lifter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:15:46Z"
concept_id: crates/re-forge/src/cuda/neural_lifter/lift_kernel_1
language: rust
---

# lift_kernel

## Signature

```rust
pub fn lift_kernel(
        &self,
        kernel: &CudaKernel,
        analysis: &PtxAnalysis,
    ) -> Result<CudaReconstructionResult>
```

## Visibility

- `pub`

## Source
Lines 30–125 in `crates/re-forge/src/cuda/neural_lifter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [neural_lifter](/crates/re-forge/src/cuda/neural_lifter.md) |
