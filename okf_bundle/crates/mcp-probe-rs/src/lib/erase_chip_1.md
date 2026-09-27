---
okf_version: "0.2"
type: Function
title: erase_chip
description: "[tool(description = \"Erase the target chip flash (requires human confirmation)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/erase_chip_1
language: rust
---

# erase_chip

[tool(description = "Erase the target chip flash (requires human confirmation)")]

## Signature

```rust
fn erase_chip(
        &self,
        Parameters(input): Parameters<EraseChipInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Erase the target chip flash (requires human confirmation)")`

## Docstring

[tool(description = "Erase the target chip flash (requires human confirmation)")]

## Source
Lines 252–279 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
