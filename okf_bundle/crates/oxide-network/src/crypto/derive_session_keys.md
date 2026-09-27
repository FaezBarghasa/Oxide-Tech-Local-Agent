---
okf_version: "0.2"
type: Function
title: derive_session_keys
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/derive_session_keys
language: rust
---

# derive_session_keys

## Signature

```rust
impl EphemeralHandshake { pub fn derive_session_keys(
        &self,
        local_priv: &DeviceIdentityKey,
        peer_pub: &DeviceIdentityPublicKey,
    ) -> Result<(SessionKey, SessionKey), OxideError> }
```

## Visibility

- `pub`

## Source
Lines 288–308 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
