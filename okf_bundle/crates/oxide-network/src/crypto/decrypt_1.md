---
okf_version: "0.2"
type: Function
title: decrypt
description: Decrypts ciphertext packet and validates authentication tag
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/decrypt_1
language: rust
---

# decrypt

Decrypts ciphertext packet and validates authentication tag

## Signature

```rust
pub fn decrypt(&self, seq: u64, ciphertext_with_tag: &[u8]) -> Result<Vec<u8>, OxideError>
```

## Visibility

- `pub`

## Docstring

Decrypts ciphertext packet and validates authentication tag

## Source
Lines 212–254 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
