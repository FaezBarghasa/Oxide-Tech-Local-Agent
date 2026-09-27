---
okf_version: "0.2"
type: Function
title: evaluate_operation
description: "Evaluates a parametric CAD operation safely, catching topological or geometric errors."
resource: crates/visual-forge/src/shadow_kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/shadow_kernel/evaluate_operation_1
language: rust
---

# evaluate_operation

Evaluates a parametric CAD operation safely, catching topological or geometric errors.

## Signature

```rust
pub fn evaluate_operation(
        &mut self,
        op: &CadOperation,
    ) -> Result<&SolidRepresentation, KernelError>
```

## Visibility

- `pub`

## Docstring

Evaluates a parametric CAD operation safely, catching topological or geometric errors.

## Source
Lines 76–335 in `crates/visual-forge/src/shadow_kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_kernel](/crates/visual-forge/src/shadow_kernel.md) |
