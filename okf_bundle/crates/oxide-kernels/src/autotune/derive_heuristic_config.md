---
okf_version: "0.2"
type: Function
title: derive_heuristic_config
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/derive_heuristic_config
language: rust
---

# derive_heuristic_config

## Signature

```rust
impl GpuAutotuner { fn derive_heuristic_config(&self, m: usize, n: usize, k: usize, is_fp8: bool) -> KernelConfig }
```

## Source
Lines 142–221 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
