---
okf_version: "0.2"
type: Function
title: verify
resource: crates/forge-rust/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/verifier/verify
language: rust
---

# verify

## Signature

```rust
impl RustVerifier { pub fn verify(code: &str) -> Result<(), VerificationError> }
```

## Visibility

- `pub`

## Source
Lines 13–19 in `crates/forge-rust/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/forge-rust/src/verifier.md) |
