---
okf_version: "0.2"
type: Function
title: check_rate_limit
resource: crates/oxide-security/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-23T19:19:18Z"
concept_id: crates/oxide-security/src/lib/check_rate_limit
language: rust
---

# check_rate_limit

## Signature

```rust
impl SecurityManager { pub fn check_rate_limit(&self, key_id: &str, max_tpm: u32, tokens: u32) -> bool }
```

## Visibility

- `pub`

## Source
Lines 75–81 in `crates/oxide-security/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-security/src/lib.md) |
