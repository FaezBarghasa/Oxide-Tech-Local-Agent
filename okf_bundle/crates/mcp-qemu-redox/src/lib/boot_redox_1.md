---
okf_version: "0.2"
type: Function
title: boot_redox
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/boot_redox_1
language: rust
---

# boot_redox

## Signature

```rust
fn boot_redox(
        &self,
        Parameters(input): Parameters<BootInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(
        description = "Boot a Redox OS image in QEMU (headless). Returns success if QEMU started."
    )`

## Source
Lines 114–122 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
