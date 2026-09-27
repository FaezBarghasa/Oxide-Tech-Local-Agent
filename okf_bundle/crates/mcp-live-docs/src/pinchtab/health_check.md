---
okf_version: "0.2"
type: Function
title: health_check
description: Check if PinchTab daemon is reachable
resource: crates/mcp-live-docs/src/pinchtab.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/mcp-live-docs/src/pinchtab/health_check
language: rust
---

# health_check

Check if PinchTab daemon is reachable

## Signature

```rust
impl PinchTabClient { pub fn health_check(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if PinchTab daemon is reachable

## Source
Lines 70–80 in `crates/mcp-live-docs/src/pinchtab.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pinchtab](/crates/mcp-live-docs/src/pinchtab.md) |
