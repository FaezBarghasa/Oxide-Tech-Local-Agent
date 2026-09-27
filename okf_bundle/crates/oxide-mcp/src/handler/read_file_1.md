---
okf_version: "0.2"
type: Function
title: read_file
description: "[tool(description = \"Read a file from the workspace\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/read_file_1
language: rust
---

# read_file

[tool(description = "Read a file from the workspace")]

## Signature

```rust
fn read_file(
        &self,
        Parameters(input): Parameters<ReadFileInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Read a file from the workspace")`

## Docstring

[tool(description = "Read a file from the workspace")]

## Source
Lines 269–281 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
