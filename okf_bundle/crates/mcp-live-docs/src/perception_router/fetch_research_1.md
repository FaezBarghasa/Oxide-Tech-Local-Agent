---
okf_version: "0.2"
type: Function
title: fetch_research
description: "Primary intelligent perception dispatch: Tier 1 (Scrapling/DocsRs) -> Tier 2 (PinchTab) -> Tier 3 (Kitesurf)"
resource: crates/mcp-live-docs/src/perception_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/mcp-live-docs/src/perception_router/fetch_research_1
language: rust
---

# fetch_research

Primary intelligent perception dispatch: Tier 1 (Scrapling/DocsRs) -> Tier 2 (PinchTab) -> Tier 3 (Kitesurf)

## Signature

```rust
pub fn fetch_research(
        &self,
        url: &str,
        force_engine: Option<&str>,
    ) -> Result<ResearchResult>
```

## Visibility

- `pub`

## Docstring

Primary intelligent perception dispatch: Tier 1 (Scrapling/DocsRs) -> Tier 2 (PinchTab) -> Tier 3 (Kitesurf)

## Source
Lines 49–73 in `crates/mcp-live-docs/src/perception_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [perception_router](/crates/mcp-live-docs/src/perception_router.md) |
