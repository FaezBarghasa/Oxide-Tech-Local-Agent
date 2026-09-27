---
okf_version: "0.2"
type: Function
title: register_tool
resource: crates/self-evolver/src/tool_maker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/self-evolver/src/tool_maker/register_tool
language: rust
---

# register_tool

## Signature

```rust
impl WasmToolRegistry { pub fn register_tool(&mut self, name: &str, wasm_bytes: Vec<u8>) }
```

## Visibility

- `pub`

## Source
Lines 313–315 in `crates/self-evolver/src/tool_maker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_maker](/crates/self-evolver/src/tool_maker.md) |
