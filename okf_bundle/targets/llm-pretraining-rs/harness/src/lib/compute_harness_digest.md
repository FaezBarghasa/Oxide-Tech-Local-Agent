---
okf_version: "0.2"
type: Function
title: compute_harness_digest
description: Computes the deterministic SHA256 digest of harness + toolchain + CUDA + ABI
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/compute_harness_digest
language: rust
---

# compute_harness_digest

Computes the deterministic SHA256 digest of harness + toolchain + CUDA + ABI

## Signature

```rust
pub fn compute_harness_digest() -> String
```

## Visibility

- `pub`

## Docstring

Computes the deterministic SHA256 digest of harness + toolchain + CUDA + ABI

## Source
Lines 102–107 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
| called_by | [default](/targets/llm-pretraining-rs/harness/src/lib/default.md) |
