---
okf_version: "0.2"
type: Function
title: local_nonce
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/local_nonce
language: rust
---

# local_nonce

## Signature

```rust
impl EphemeralHandshake { pub fn local_nonce(&self) -> &[u8; 32] }
```

## Visibility

- `pub`

## Source
Lines 280–282 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
