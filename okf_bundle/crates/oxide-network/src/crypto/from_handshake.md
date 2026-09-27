---
okf_version: "0.2"
type: Function
title: from_handshake
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/from_handshake
language: rust
---

# from_handshake

## Signature

```rust
impl SessionKey { pub fn from_handshake(shared_secret: &[u8], salt: &[u8], context: &str) -> Self }
```

## Visibility

- `pub`

## Source
Lines 147–155 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
