---
okf_version: "0.2"
type: Function
title: list_tools
description: List all tools exposed by the MCP server.
resource: crates/mcp-clients/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-clients"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/mcp-clients/src/lib/list_tools_1
language: rust
---

# list_tools

List all tools exposed by the MCP server.

## Signature

```rust
pub fn list_tools(&self) -> Result<ListToolsResult>
```

## Visibility

- `pub`

## Docstring

List all tools exposed by the MCP server.

## Source
Lines 59–64 in `crates/mcp-clients/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-clients/src/lib.md) |
