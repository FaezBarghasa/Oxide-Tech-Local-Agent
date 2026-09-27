---
okf_version: "0.2"
type: Function
title: cargo_check
description: "[tool(description = \"Run cargo check inside the native sandbox\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/cargo_check_1
language: rust
---

# cargo_check

[tool(description = "Run cargo check inside the native sandbox")]

## Signature

```rust
fn cargo_check(
        &self,
        Parameters(input): Parameters<CargoCheckInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Run cargo check inside the native sandbox")`

## Docstring

[tool(description = "Run cargo check inside the native sandbox")]

## Source
Lines 337–364 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
