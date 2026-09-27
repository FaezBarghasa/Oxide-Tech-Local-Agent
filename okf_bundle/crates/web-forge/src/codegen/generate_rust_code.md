---
okf_version: "0.2"
type: Function
title: generate_rust_code
description: "Generates idiomatic Rust asynchronous test/execution code for a given `WebScript`."
resource: crates/web-forge/src/codegen.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:21Z"
concept_id: crates/web-forge/src/codegen/generate_rust_code
language: rust
---

# generate_rust_code

Generates idiomatic Rust asynchronous test/execution code for a given `WebScript`.

## Signature

```rust
impl MacroCodegen { pub fn generate_rust_code(script: &WebScript) -> String }
```

## Visibility

- `pub`

## Docstring

Generates idiomatic Rust asynchronous test/execution code for a given `WebScript`.

## Source
Lines 67–115 in `crates/web-forge/src/codegen.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codegen](/crates/web-forge/src/codegen.md) |
