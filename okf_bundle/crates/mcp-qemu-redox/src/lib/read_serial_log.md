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
concept_id: crates/mcp-qemu-redox/src/lib/read_serial_log
language: rust
---

# read_serial_log

[tool(description = "Read accumulated QEMU serial log.")]

## Signature

```rust
impl QemuRedoxServer { fn read_serial_log(
        &self,
        _input: Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Read accumulated QEMU serial log.")]

## Source
Lines 156–163 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
