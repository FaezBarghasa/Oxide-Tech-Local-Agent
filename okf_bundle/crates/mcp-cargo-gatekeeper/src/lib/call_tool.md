---
okf_version: "0.2"
type: Function
title: call_tool
resource: crates/mcp-cargo-gatekeeper/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-cargo-gatekeeper"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:12Z"
concept_id: crates/mcp-cargo-gatekeeper/src/lib/call_tool
language: rust
---

# call_tool

## Signature

```rust
impl CargoGatekeeperServer { fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> }
```

## Source
Lines 206–213 in `crates/mcp-cargo-gatekeeper/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-cargo-gatekeeper/src/lib.md) |
