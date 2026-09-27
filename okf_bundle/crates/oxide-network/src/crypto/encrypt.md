---
okf_version: "0.2"
type: Function
title: encrypt
description: Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/encrypt
language: rust
---

# encrypt

Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag

## Signature

```rust
impl AeadCipher { pub fn encrypt(&self, seq: u64, plaintext: &[u8]) -> Vec<u8> }
```

## Visibility

- `pub`

## Docstring

Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag

## Source
Lines 179–209 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
