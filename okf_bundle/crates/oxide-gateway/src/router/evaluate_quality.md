---
okf_version: "0.2"
type: Function
title: evaluate_quality
description: Evaluate cargo-check output and decide whether to flip the backend.
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/evaluate_quality
language: rust
---

# evaluate_quality

Evaluate cargo-check output and decide whether to flip the backend.

## Signature

```rust
impl GatewayRouter { pub fn evaluate_quality(&self, stderr: &str, current: CoderBackend) -> Option<CoderBackend> }
```

## Visibility

- `pub`

## Docstring

Evaluate cargo-check output and decide whether to flip the backend.

Returns `Some(CoderBackend)` with a recommended fallback if the current
backend is deemed "unsatisfying", or `None` if quality is acceptable.

## Source
Lines 127–155 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
