---
okf_version: "0.2"
type: Function
title: fetch_crate_docs
description: "[tool(description = \"Fetch and index crate docs from docs.rs for a specific version\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/fetch_crate_docs_1
language: rust
---

# fetch_crate_docs

[tool(description = "Fetch and index crate docs from docs.rs for a specific version")]

## Signature

```rust
fn fetch_crate_docs(
        &self,
        Parameters(input): Parameters<FetchCrateDocsInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Fetch and index crate docs from docs.rs for a specific version")`

## Docstring

[tool(description = "Fetch and index crate docs from docs.rs for a specific version")]

## Source
Lines 425–477 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
| calls | [fetch_latest_crates_io_version](/crates/rag-pipeline/src/updater/fetch_latest_crates_io_version.md) |
