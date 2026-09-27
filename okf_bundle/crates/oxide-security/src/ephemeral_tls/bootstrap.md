---
okf_version: "0.2"
type: Function
title: bootstrap
description: "Bootstrap an ephemeral, in-memory mTLS context bound to local system entropy."
resource: crates/oxide-security/src/ephemeral_tls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-23T19:18:11Z"
concept_id: crates/oxide-security/src/ephemeral_tls/bootstrap
language: rust
---

# bootstrap

Bootstrap an ephemeral, in-memory mTLS context bound to local system entropy.

## Signature

```rust
impl EphemeralTlsContext { pub fn bootstrap() -> Result<Self, SecurityError> }
```

## Visibility

- `pub`

## Docstring

Bootstrap an ephemeral, in-memory mTLS context bound to local system entropy.

## Source
Lines 28–86 in `crates/oxide-security/src/ephemeral_tls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral_tls](/crates/oxide-security/src/ephemeral_tls.md) |
