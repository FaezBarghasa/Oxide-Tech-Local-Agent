---
okf_version: "0.2"
type: Function
title: measure
resource: crates/cad-forge/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/kernel/measure_2
language: rust
---

# measure

## Signature

```rust
impl OcctBackend { fn measure(&self, b: &Body, q: MeasureQuery) -> Result<f64, CadKernelError> }
```

## Source
Lines 240–242 in `crates/cad-forge/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/cad-forge/src/kernel.md) |
