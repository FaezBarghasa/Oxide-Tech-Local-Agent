---
okf_version: "0.2"
type: Function
title: execute_wasm_sandbox
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/execute_wasm_sandbox
language: rust
---

# execute_wasm_sandbox

## Signature

```rust
pub fn execute_wasm_sandbox(
    wasm_bytes: &[u8],
    _inputs: &[u8],
    _fuel: u64,
) -> Result<ExecutionResult, String>
```

## Visibility

- `pub`

## Source
Lines 255–269 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
| called_by | [test_wasm_tool](/crates/self-evolver/src/tool_maker/test_wasm_tool.md) |
