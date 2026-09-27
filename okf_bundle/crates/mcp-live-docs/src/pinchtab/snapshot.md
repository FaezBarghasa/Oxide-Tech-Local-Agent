---
okf_version: "0.2"
type: Function
title: snapshot
description: Get accessibility-tree based page snapshot
resource: crates/mcp-live-docs/src/pinchtab.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/mcp-live-docs/src/pinchtab/snapshot
language: rust
---

# snapshot

Get accessibility-tree based page snapshot

## Signature

```rust
impl PinchTabClient { pub fn snapshot(&self, tab_id: Option<&str>) -> Result<SnapshotResponse> }
```

## Visibility

- `pub`

## Docstring

Get accessibility-tree based page snapshot

## Source
Lines 123–136 in `crates/mcp-live-docs/src/pinchtab.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pinchtab](/crates/mcp-live-docs/src/pinchtab.md) |
