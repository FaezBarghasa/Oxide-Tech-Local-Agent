---
okf_version: "0.2"
type: Function
title: compile_rust_to_wasm
description: Compile a Rust source string into a standalone Wasm binary
resource: crates/sandbox/src/wasm_compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/sandbox/src/wasm_compiler/compile_rust_to_wasm
language: rust
---

# compile_rust_to_wasm

Compile a Rust source string into a standalone Wasm binary

## Signature

```rust
impl WasmCompiler { pub fn compile_rust_to_wasm(&self, crate_name: &str, source: &str) -> Result<PathBuf> }
```

## Visibility

- `pub`

## Docstring

Compile a Rust source string into a standalone Wasm binary

## Source
Lines 17–35 in `crates/sandbox/src/wasm_compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wasm_compiler](/crates/sandbox/src/wasm_compiler.md) |
