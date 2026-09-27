---
okf_version: "0.2"
type: Function
title: call_tool_with_dtx
description: Call a specific tool with DTX header for distributed transaction tracking
resource: crates/mcp-clients/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-clients"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/mcp-clients/src/lib/call_tool_with_dtx
language: rust
---

# call_tool_with_dtx

Call a specific tool with DTX header for distributed transaction tracking

## Signature

```rust
impl EiosMcpClient { pub fn call_tool_with_dtx(
        &self,
        name: &str,
        arguments: serde_json::Value,
        dtx_id: Option<oxide_protocol::DtxId>,
    ) -> Result<CallToolResult> }
```

## Visibility

- `pub`

## Docstring

Call a specific tool with DTX header for distributed transaction tracking

## Source
Lines 76–100 in `crates/mcp-clients/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-clients/src/lib.md) |
