---
okf_version: "0.2"
type: Function
title: live_docs_scrape
description: "[tool(description = \"Scrape docs.rs for crate updates and updates RAG index\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/live_docs_scrape_1
language: rust
---

# live_docs_scrape

[tool(description = "Scrape docs.rs for crate updates and updates RAG index")]

## Signature

```rust
fn live_docs_scrape(
        &self,
        Parameters(input): Parameters<LiveDocsScrapeInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Scrape docs.rs for crate updates and updates RAG index")`

## Docstring

[tool(description = "Scrape docs.rs for crate updates and updates RAG index")]

## Source
Lines 605–626 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
