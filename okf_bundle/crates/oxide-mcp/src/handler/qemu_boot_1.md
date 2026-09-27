---
okf_version: "0.2"
type: Function
title: qemu_boot
description: "[tool(description = \"Boot OS image in QEMU\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/qemu_boot_1
language: rust
---

# qemu_boot

[tool(description = "Boot OS image in QEMU")]

## Signature

```rust
fn qemu_boot(
        &self,
        Parameters(input): Parameters<QemuBootInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Boot OS image in QEMU")`

## Docstring

[tool(description = "Boot OS image in QEMU")]

## Source
Lines 543–552 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
