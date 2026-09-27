---
okf_version: "0.2"
type: Function
title: list_tools
resource: crates/mcp-live-docs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/mcp-live-docs/src/lib/list_tools_1
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
Lines 210–221 in `crates/mcp-live-docs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-live-docs/src/lib.md) |
