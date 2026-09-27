---
okf_version: "0.2"
type: Function
title: test_wasm_tool
description: Test a synthesized WebAssembly tool inside the Wasm sandbox
resource: crates/self-evolver/src/tool_maker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/self-evolver/src/tool_maker/test_wasm_tool_1
language: rust
---

# test_wasm_tool

Test a synthesized WebAssembly tool inside the Wasm sandbox

## Signature

```rust
pub fn test_wasm_tool(
        &self,
        wasm_bytes: &[u8],
        sample_inputs: &[u8],
    ) -> Result<TestResult>
```

## Visibility

- `pub`

## Docstring

Test a synthesized WebAssembly tool inside the Wasm sandbox

## Source
Lines 280–297 in `crates/self-evolver/src/tool_maker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_maker](/crates/self-evolver/src/tool_maker.md) |
| calls | [execute_wasm_sandbox](/crates/sandbox/src/execution/execute_wasm_sandbox.md) |
