---
okf_version: "0.2"
type: Function
title: run_panic_test
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/run_panic_test_1
language: rust
---

# run_panic_test

## Signature

```rust
fn run_panic_test(
        &self,
        Parameters(input): Parameters<PanicTestInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(
        description = "Run a kernel panic test: boot image, send trigger command, capture and parse stack trace."
    )`

## Source
Lines 187–216 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
