---
okf_version: "0.2"
type: Function
title: list_tools
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/list_tools_1
language: rust
---

# list_tools

## Signature

```rust
fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError>
```

## Source
Lines 287–298 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
