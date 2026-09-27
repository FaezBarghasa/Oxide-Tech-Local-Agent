---
okf_version: "0.2"
type: Function
title: from_wat
resource: crates/wasm-forge/src/wasm_emitter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:wasm-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:54:27Z"
concept_id: crates/wasm-forge/src/wasm_emitter/from_wat
language: rust
---

# from_wat

## Signature

```rust
impl WasmModule { pub fn from_wat(name: impl Into<String>, wat_source: &str) -> Result<Self, WasmError> }
```

## Visibility

- `pub`

## Source
Lines 44–51 in `crates/wasm-forge/src/wasm_emitter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wasm_emitter](/crates/wasm-forge/src/wasm_emitter.md) |
