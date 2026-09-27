---
okf_version: "0.2"
type: Function
title: send_uart
description: "[tool(description = \"Send a line to QEMU UART (stdin).\")]"
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/send_uart
language: rust
---

# send_uart

[tool(description = "Send a line to QEMU UART (stdin).")]

## Signature

```rust
impl QemuRedoxServer { fn send_uart(
        &self,
        Parameters(input): Parameters<UartInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Send a line to QEMU UART (stdin).")]

## Source
Lines 125–153 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
