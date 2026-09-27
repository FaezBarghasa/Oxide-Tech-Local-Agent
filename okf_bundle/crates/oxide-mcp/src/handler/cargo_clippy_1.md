---
okf_version: "0.2"
type: Function
title: cargo_clippy
description: "[tool(description = \"Run cargo clippy inside the native sandbox\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/cargo_clippy_1
language: rust
---

# cargo_clippy

[tool(description = "Run cargo clippy inside the native sandbox")]

## Signature

```rust
fn cargo_clippy(
        &self,
        Parameters(input): Parameters<CargoClippyInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Run cargo clippy inside the native sandbox")`

## Docstring

[tool(description = "Run cargo clippy inside the native sandbox")]

## Source
Lines 367–394 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
