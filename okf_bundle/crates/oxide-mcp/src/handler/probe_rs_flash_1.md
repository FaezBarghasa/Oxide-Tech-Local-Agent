---
okf_version: "0.2"
type: Function
title: probe_rs_flash
description: "[tool(description = \"Flash binary to hardware using probe-rs (requires human confirmation)\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/probe_rs_flash_1
language: rust
---

# probe_rs_flash

[tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")]

## Signature

```rust
fn probe_rs_flash(
        &self,
        Parameters(input): Parameters<ProbeRsFlashInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")`

## Docstring

[tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")]

## Source
Lines 511–528 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
