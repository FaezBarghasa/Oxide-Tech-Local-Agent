---
okf_version: "0.2"
type: Function
title: boolean
resource: crates/cad-forge/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/kernel/boolean_2
language: rust
---

# boolean

## Signature

```rust
impl OcctBackend { fn boolean(&self, op: BooleanOpKind, a: &Body, b: &Body) -> Result<Body, CadKernelError> }
```

## Source
Lines 232–234 in `crates/cad-forge/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/cad-forge/src/kernel.md) |
