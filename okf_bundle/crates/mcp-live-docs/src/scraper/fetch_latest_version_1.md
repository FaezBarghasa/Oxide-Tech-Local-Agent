---
okf_version: "0.2"
type: Function
title: fetch_latest_version
description: "Resolve the latest version from crates.io. Returns a string like \"1.2.3\"."
resource: crates/mcp-live-docs/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:51:07Z"
concept_id: crates/mcp-live-docs/src/scraper/fetch_latest_version_1
language: rust
---

# fetch_latest_version

Resolve the latest version from crates.io. Returns a string like "1.2.3".

## Signature

```rust
pub fn fetch_latest_version(&self, crate_name: &str) -> Result<String>
```

## Visibility

- `pub`

## Docstring

Resolve the latest version from crates.io. Returns a string like "1.2.3".

## Source
Lines 57–72 in `crates/mcp-live-docs/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/mcp-live-docs/src/scraper.md) |
