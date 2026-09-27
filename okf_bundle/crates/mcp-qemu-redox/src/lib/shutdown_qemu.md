---
okf_version: "0.2"
type: Function
title: shutdown_qemu
description: "[tool(description = \"Shutdown the running QEMU instance.\")]"
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/shutdown_qemu
language: rust
---

# shutdown_qemu

[tool(description = "Shutdown the running QEMU instance.")]

## Signature

```rust
impl QemuRedoxServer { fn shutdown_qemu(
        &self,
        _input: Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Shutdown the running QEMU instance.")]

## Source
Lines 166–182 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
