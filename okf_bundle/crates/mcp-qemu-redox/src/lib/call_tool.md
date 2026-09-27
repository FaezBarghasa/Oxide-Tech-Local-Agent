---
okf_version: "0.2"
type: Function
title: call_tool
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/call_tool
language: rust
---

# call_tool

## Signature

```rust
impl QemuRedoxServer { fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> }
```

## Source
Lines 237–244 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
