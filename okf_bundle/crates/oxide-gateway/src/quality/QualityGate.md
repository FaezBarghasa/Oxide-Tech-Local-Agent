---
okf_version: "0.2"
type: Class
title: QualityGate
description: "Parses `cargo check` stderr to count Rust compiler errors and produce a"
resource: crates/oxide-gateway/src/quality.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/quality/QualityGate
language: rust
---

# QualityGate

Parses `cargo check` stderr to count Rust compiler errors and produce a

## Signature

```rust
pub struct QualityGate
```

## Visibility

- `pub`

## Docstring

Parses `cargo check` stderr to count Rust compiler errors and produce a
normalised quality score in [0.0, 1.0].

A score of 1.0 means the output is error-free.
A score of 0.0 means `max_errors` or more errors were found.

## Methods

- `max_errors`
- `threshold`

## Source
Lines 8–14 in `crates/oxide-gateway/src/quality.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [quality](/crates/oxide-gateway/src/quality.md) |
