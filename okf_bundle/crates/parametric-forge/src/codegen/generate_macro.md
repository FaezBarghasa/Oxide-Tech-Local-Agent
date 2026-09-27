---
okf_version: "0.2"
type: Function
title: generate_macro
description: "Synthesizes a `DeltaPatch` or sequence of `CadOperation`s into a Rust `nexus_macro::parametric_edit!` invocation string."
resource: crates/parametric-forge/src/codegen.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/codegen/generate_macro
language: rust
---

# generate_macro

Synthesizes a `DeltaPatch` or sequence of `CadOperation`s into a Rust `nexus_macro::parametric_edit!` invocation string.

## Signature

```rust
impl MacroCodegen { pub fn generate_macro(
        dag: &FeatureDAG,
        patches: &[DeltaPatch],
    ) -> Result<String, ParametricError> }
```

## Visibility

- `pub`

## Docstring

Synthesizes a `DeltaPatch` or sequence of `CadOperation`s into a Rust `nexus_macro::parametric_edit!` invocation string.

## Source
Lines 10–106 in `crates/parametric-forge/src/codegen.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codegen](/crates/parametric-forge/src/codegen.md) |
