---
okf_version: "0.2"
type: Function
title: export_pdf
description: Convert page/whitepaper to PDF via Cloudflare Browser Run
resource: crates/mcp-live-docs/src/kitesurf.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/mcp-live-docs/src/kitesurf/export_pdf_1
language: rust
---

# export_pdf

Convert page/whitepaper to PDF via Cloudflare Browser Run

## Signature

```rust
pub fn export_pdf(&self, url: &str) -> Result<Vec<u8>>
```

## Visibility

- `pub`

## Docstring

Convert page/whitepaper to PDF via Cloudflare Browser Run

## Source
Lines 147–172 in `crates/mcp-live-docs/src/kitesurf.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kitesurf](/crates/mcp-live-docs/src/kitesurf.md) |
