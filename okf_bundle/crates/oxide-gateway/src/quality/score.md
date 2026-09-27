---
okf_version: "0.2"
type: Function
title: score
description: "Score the output of a `cargo check` / `cargo clippy` run."
resource: crates/oxide-gateway/src/quality.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/quality/score
language: rust
---

# score

Score the output of a `cargo check` / `cargo clippy` run.

## Signature

```rust
impl QualityGate { pub fn score(&self, stderr: &str) -> (f32, usize) }
```

## Visibility

- `pub`

## Docstring

Score the output of a `cargo check` / `cargo clippy` run.

`stderr` — the captured standard error from the Cargo invocation.
Returns `(score, error_count)`.

## Source
Lines 29–45 in `crates/oxide-gateway/src/quality.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [quality](/crates/oxide-gateway/src/quality.md) |
| calls | [count_compiler_errors](/crates/oxide-gateway/src/quality/count_compiler_errors.md) |
