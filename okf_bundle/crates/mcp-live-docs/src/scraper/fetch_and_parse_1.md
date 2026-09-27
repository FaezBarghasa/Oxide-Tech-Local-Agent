---
okf_version: "0.2"
type: Function
title: fetch_and_parse
description: Fetch the docs.rs index page for the given crate+version and parse an API surface.
resource: crates/mcp-live-docs/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:51:07Z"
concept_id: crates/mcp-live-docs/src/scraper/fetch_and_parse_1
language: rust
---

# fetch_and_parse

Fetch the docs.rs index page for the given crate+version and parse an API surface.

## Signature

```rust
pub fn fetch_and_parse(&self, crate_name: &str, version: &str) -> Result<ApiSurface>
```

## Visibility

- `pub`

## Docstring

Fetch the docs.rs index page for the given crate+version and parse an API surface.

## Source
Lines 75–83 in `crates/mcp-live-docs/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/mcp-live-docs/src/scraper.md) |
