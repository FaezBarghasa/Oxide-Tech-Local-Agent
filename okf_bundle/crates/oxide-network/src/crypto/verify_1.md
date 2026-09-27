---
okf_version: "0.2"
type: Function
title: verify
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/verify_1
language: rust
---

# verify

## Signature

```rust
pub fn verify(&self, msg: &[u8], sig: &DeviceSignature) -> Result<(), OxideError>
```

## Visibility

- `pub`

## Source
Lines 84–89 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
