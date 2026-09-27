---
okf_version: "0.2"
type: Function
title: verify_key
resource: crates/oxide-security/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-23T19:19:18Z"
concept_id: crates/oxide-security/src/lib/verify_key
language: rust
---

# verify_key

## Signature

```rust
impl SecurityManager { pub fn verify_key(raw_key: &str, hash: &str) -> bool }
```

## Visibility

- `pub`

## Source
Lines 70–73 in `crates/oxide-security/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-security/src/lib.md) |
