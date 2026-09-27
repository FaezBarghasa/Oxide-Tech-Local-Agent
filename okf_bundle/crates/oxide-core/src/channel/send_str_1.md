---
okf_version: "0.2"
type: Function
title: send_str
description: Send a UTF-8 token string converted directly to static/borrowed bytes.
resource: crates/oxide-core/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/channel/send_str_1
language: rust
---

# send_str

Send a UTF-8 token string converted directly to static/borrowed bytes.

## Signature

```rust
pub fn send_str(
        &self,
        text: impl Into<String>,
    ) -> Result<(), mpsc::error::SendError<Bytes>>
```

## Visibility

- `pub`

## Docstring

Send a UTF-8 token string converted directly to static/borrowed bytes.

## Source
Lines 27–33 in `crates/oxide-core/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-core/src/channel.md) |
