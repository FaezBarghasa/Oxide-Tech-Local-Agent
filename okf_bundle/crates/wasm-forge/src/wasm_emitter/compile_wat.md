---
okf_version: "0.2"
type: Function
title: compile_wat
description: Compile WAT source text to binary WASM bytecode.
resource: crates/wasm-forge/src/wasm_emitter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:wasm-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:54:27Z"
concept_id: crates/wasm-forge/src/wasm_emitter/compile_wat
language: rust
---

# compile_wat

Compile WAT source text to binary WASM bytecode.

## Signature

```rust
impl WasmEngine { pub fn compile_wat(&self, wat_source: &str) -> Result<Vec<u8>, WasmError> }
```

## Visibility

- `pub`

## Docstring

Compile WAT source text to binary WASM bytecode.

## Source
Lines 33–35 in `crates/wasm-forge/src/wasm_emitter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wasm_emitter](/crates/wasm-forge/src/wasm_emitter.md) |
