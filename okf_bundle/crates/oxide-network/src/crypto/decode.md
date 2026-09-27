---
okf_version: "0.2"
type: Function
title: decode
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/decode
language: rust
---

# decode

## Signature

```rust
pub fn decode(hex_str: &str) -> Result<Vec<u8>, String>
```

## Visibility

- `pub`

## Source
Lines 320–331 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
| called_by | [deserialize](/crates/oxide-network/src/crypto/deserialize.md) |
