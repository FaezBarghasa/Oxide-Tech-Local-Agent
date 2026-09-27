---
okf_version: "0.2"
type: Function
title: read_serial_log
description: "[tool(description = \"Read accumulated QEMU serial log.\")]"
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/read_serial_log_1
language: rust
---

# read_serial_log

[tool(description = "Read accumulated QEMU serial log.")]

## Signature

```rust
fn read_serial_log(
        &self,
        _input: Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Read accumulated QEMU serial log.")`

## Docstring

[tool(description = "Read accumulated QEMU serial log.")]

## Source
Lines 156–163 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
