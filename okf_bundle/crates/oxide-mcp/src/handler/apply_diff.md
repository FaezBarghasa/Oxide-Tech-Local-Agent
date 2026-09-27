---
okf_version: "0.2"
type: Function
title: apply_diff
description: "[tool(description = \"Apply a search and replace diff to a file in the workspace\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/apply_diff
language: rust
---

# apply_diff

[tool(description = "Apply a search and replace diff to a file in the workspace")]

## Signature

```rust
impl McpServer { fn apply_diff(
        &self,
        Parameters(input): Parameters<ApplyDiffInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Apply a search and replace diff to a file in the workspace")]

## Source
Lines 305–334 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
