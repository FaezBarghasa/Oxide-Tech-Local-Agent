---
okf_version: "0.2"
type: Function
title: qemu_send_uart
description: "[tool(description = \"Send command to QEMU serial port UART\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/qemu_send_uart
language: rust
---

# qemu_send_uart

[tool(description = "Send command to QEMU serial port UART")]

## Signature

```rust
impl McpServer { fn qemu_send_uart(
        &self,
        Parameters(input): Parameters<QemuUartInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Send command to QEMU serial port UART")]

## Source
Lines 555–564 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
