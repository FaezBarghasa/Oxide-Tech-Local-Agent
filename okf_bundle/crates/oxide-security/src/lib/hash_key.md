---
okf_version: "0.2"
type: Function
title: hash_key
resource: crates/oxide-security/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-23T19:19:18Z"
concept_id: crates/oxide-security/src/lib/hash_key
language: rust
---

# hash_key

## Signature

```rust
impl SecurityManager { pub fn hash_key(raw_key: &str) -> Result<String, String> }
```

## Visibility

- `pub`

## Source
Lines 65–68 in `crates/oxide-security/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-security/src/lib.md) |
