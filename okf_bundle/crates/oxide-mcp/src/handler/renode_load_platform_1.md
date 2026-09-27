---
okf_version: "0.2"
type: Function
title: renode_load_platform
description: "[tool(description = \"Load platform description script in Renode\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/renode_load_platform_1
language: rust
---

# renode_load_platform

[tool(description = "Load platform description script in Renode")]

## Signature

```rust
fn renode_load_platform(
        &self,
        Parameters(input): Parameters<RenodeLoadInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Load platform description script in Renode")`

## Docstring

[tool(description = "Load platform description script in Renode")]

## Source
Lines 567–576 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
