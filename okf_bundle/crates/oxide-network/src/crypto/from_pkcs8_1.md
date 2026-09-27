---
okf_version: "0.2"
type: Function
title: from_pkcs8
description: Creates key from PKCS8 document bytes
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/from_pkcs8_1
language: rust
---

# from_pkcs8

Creates key from PKCS8 document bytes

## Signature

```rust
pub fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Docstring

Creates key from PKCS8 document bytes

## Source
Lines 37–43 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
