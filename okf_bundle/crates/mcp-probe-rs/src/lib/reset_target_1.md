---
okf_version: "0.2"
type: Function
title: reset_target
description: "[tool(description = \"Reset the target chip (requires human confirmation)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/reset_target_1
language: rust
---

# reset_target

[tool(description = "Reset the target chip (requires human confirmation)")]

## Signature

```rust
fn reset_target(
        &self,
        Parameters(input): Parameters<ResetTargetInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Reset the target chip (requires human confirmation)")`

## Docstring

[tool(description = "Reset the target chip (requires human confirmation)")]

## Source
Lines 222–249 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
