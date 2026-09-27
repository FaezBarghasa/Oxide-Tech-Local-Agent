---
okf_version: "0.2"
type: Function
title: generate_kani_proof_harness
description: "Generates automated `#[kani::proof]` verification harnesses for embedded Rust algorithms."
resource: crates/formal-verify/src/kani_harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:15:37Z"
concept_id: crates/formal-verify/src/kani_harness/generate_kani_proof_harness
language: rust
---

# generate_kani_proof_harness

Generates automated `#[kani::proof]` verification harnesses for embedded Rust algorithms.

## Signature

```rust
pub fn generate_kani_proof_harness(target: &KaniHarnessTarget) -> String
```

## Visibility

- `pub`

## Docstring

Generates automated `#[kani::proof]` verification harnesses for embedded Rust algorithms.

## Source
Lines 12–31 in `crates/formal-verify/src/kani_harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kani_harness](/crates/formal-verify/src/kani_harness.md) |
| called_by | [test_kani_harness_generation](/crates/formal-verify/tests/verify_tests/test_kani_harness_generation.md) |
