---
okf_version: "0.2"
type: Function
title: recv
description: Asynchronously receive the next token chunk.
resource: crates/oxide-core/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/channel/recv
language: rust
---

# recv

Asynchronously receive the next token chunk.

## Signature

```rust
impl TokenReceiver { pub fn recv(&mut self) -> Option<Bytes> }
```

## Visibility

- `pub`

## Docstring

Asynchronously receive the next token chunk.

## Source
Lines 47–49 in `crates/oxide-core/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-core/src/channel.md) |
