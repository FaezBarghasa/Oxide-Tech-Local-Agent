---
okf_version: "0.2"
type: Function
title: generate_macro
description: "Generates a reusable Rust declarative macro (`nexus_macro::...`) for instant replay."
resource: crates/scene-forge/src/codegen.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:26:52Z"
concept_id: crates/scene-forge/src/codegen/generate_macro_1
language: rust
---

# generate_macro

Generates a reusable Rust declarative macro (`nexus_macro::...`) for instant replay.

## Signature

```rust
pub fn generate_macro(macro_name: &str, commands: &[BlenderCommand]) -> String
```

## Visibility

- `pub`

## Docstring

Generates a reusable Rust declarative macro (`nexus_macro::...`) for instant replay.

## Source
Lines 9–98 in `crates/scene-forge/src/codegen.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codegen](/crates/scene-forge/src/codegen.md) |
