---
okf_version: "0.2"
type: Function
title: write_file
description: "[tool(description = \"Write a file to the workspace\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/write_file_1
language: rust
---

# write_file

[tool(description = "Write a file to the workspace")]

## Signature

```rust
fn write_file(
        &self,
        Parameters(input): Parameters<WriteFileInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Write a file to the workspace")`

## Docstring

[tool(description = "Write a file to the workspace")]

## Source
Lines 284–302 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
