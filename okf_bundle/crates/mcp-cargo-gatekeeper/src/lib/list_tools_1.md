---
okf_version: "0.2"
type: Function
title: list_tools
resource: crates/mcp-cargo-gatekeeper/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-cargo-gatekeeper"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:12Z"
concept_id: crates/mcp-cargo-gatekeeper/src/lib/list_tools_1
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
Lines 193–204 in `crates/mcp-cargo-gatekeeper/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-cargo-gatekeeper/src/lib.md) |
