---
okf_version: "0.2"
type: Function
title: test_kani_harness_generation
description: "[test]"
resource: crates/formal-verify/tests/verify_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/formal-verify/tests/verify_tests/test_kani_harness_generation
language: rust
---

# test_kani_harness_generation

[test]

## Signature

```rust
fn test_kani_harness_generation()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 45–56 in `crates/formal-verify/tests/verify_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verify_tests](/crates/formal-verify/tests/verify_tests.md) |
| calls | [generate_kani_proof_harness](/crates/formal-verify/src/kani_harness/generate_kani_proof_harness.md) |
