---
okf_version: "0.2"
type: Function
title: lookup_crate_api
description: "[tool(description = \"Lookup the public API surface of a crate (latest version if omitted)\")]"
resource: crates/mcp-live-docs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/mcp-live-docs/src/lib/lookup_crate_api
language: rust
---

# lookup_crate_api

[tool(description = "Lookup the public API surface of a crate (latest version if omitted)")]

## Signature

```rust
impl LiveDocsServer { fn lookup_crate_api(
        &self,
        Parameters(input): Parameters<CrateLookupInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Lookup the public API surface of a crate (latest version if omitted)")]

## Source
Lines 87–128 in `crates/mcp-live-docs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-live-docs/src/lib.md) |
