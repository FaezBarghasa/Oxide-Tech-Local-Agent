---
okf_version: "0.2"
type: Function
title: generate_macro
description: "Synthesizes a verified closed-loop CAD convergence result into a `nexus_macro::visual_match!` macro string."
resource: crates/visual-forge/src/codegen.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:01:01Z"
concept_id: crates/visual-forge/src/codegen/generate_macro_1
language: rust
---

# generate_macro

Synthesizes a verified closed-loop CAD convergence result into a `nexus_macro::visual_match!` macro string.

## Signature

```rust
pub fn generate_macro(
        target_name: &str,
        tolerance_mm: f32,
        iou: f64,
        steps: usize,
    ) -> Result<String, VisualForgeError>
```

## Visibility

- `pub`

## Docstring

Synthesizes a verified closed-loop CAD convergence result into a `nexus_macro::visual_match!` macro string.

## Source
Lines 8–26 in `crates/visual-forge/src/codegen.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codegen](/crates/visual-forge/src/codegen.md) |
