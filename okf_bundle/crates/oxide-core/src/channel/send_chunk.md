---
okf_version: "0.2"
type: Function
title: send_chunk
description: "Send a token chunk as `Bytes` without copying."
resource: crates/oxide-core/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/channel/send_chunk
language: rust
---

# send_chunk

Send a token chunk as `Bytes` without copying.

## Signature

```rust
impl TokenSender { pub fn send_chunk(&self, chunk: Bytes) -> Result<(), mpsc::error::SendError<Bytes>> }
```

## Visibility

- `pub`

## Docstring

Send a token chunk as `Bytes` without copying.

## Source
Lines 22–24 in `crates/oxide-core/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-core/src/channel.md) |
