---
okf_version: "0.2"
type: Function
title: poll_next
resource: crates/oxide-core/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/channel/poll_next_1
language: rust
---

# poll_next

## Signature

```rust
fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

## Source
Lines 55–57 in `crates/oxide-core/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-core/src/channel.md) |
