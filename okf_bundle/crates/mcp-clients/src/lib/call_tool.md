---
okf_version: "0.2"
type: Function
title: call_tool
description: Call a specific tool with arguments.
resource: crates/mcp-clients/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-clients"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/mcp-clients/src/lib/call_tool
language: rust
---

# call_tool

Call a specific tool with arguments.

## Signature

```rust
impl EiosMcpClient { pub fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> }
```

## Visibility

- `pub`

## Docstring

Call a specific tool with arguments.

## Source
Lines 67–73 in `crates/mcp-clients/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-clients/src/lib.md) |
