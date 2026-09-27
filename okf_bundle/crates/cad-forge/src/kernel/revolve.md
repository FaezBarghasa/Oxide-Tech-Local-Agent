---
okf_version: "0.2"
type: Function
title: revolve
resource: crates/cad-forge/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/kernel/revolve
language: rust
---

# revolve

## Signature

```rust
impl PolyBackend { fn revolve(
        &self,
        sketch: &Sketch,
        _axis: Axis,
        angle_deg: f64,
    ) -> Result<Body, CadKernelError> }
```

## Source
Lines 134–150 in `crates/cad-forge/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/cad-forge/src/kernel.md) |
