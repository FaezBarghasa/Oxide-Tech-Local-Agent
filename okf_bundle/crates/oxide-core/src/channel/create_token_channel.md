---
okf_version: "0.2"
type: Function
title: create_token_channel
description: Create a bounded token stream channel with pre-allocated buffer slots.
resource: crates/oxide-core/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/channel/create_token_channel
language: rust
---

# create_token_channel

Create a bounded token stream channel with pre-allocated buffer slots.

## Signature

```rust
pub fn create_token_channel(capacity: usize) -> (TokenSender, TokenReceiver)
```

## Visibility

- `pub`

## Docstring

Create a bounded token stream channel with pre-allocated buffer slots.

## Source
Lines 61–64 in `crates/oxide-core/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-core/src/channel.md) |
| called_by | [test_token_channel_streaming](/crates/oxide-core/src/channel/test_token_channel_streaming.md) |
