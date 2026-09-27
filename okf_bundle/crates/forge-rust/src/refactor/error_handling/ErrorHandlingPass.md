---
okf_version: "0.2"
type: Class
title: ErrorHandlingPass
description: "Transforms nullable types, error integer codes (-1 / errno), and exceptions"
resource: crates/forge-rust/src/refactor/error_handling.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T06:58:29Z"
concept_id: crates/forge-rust/src/refactor/error_handling/ErrorHandlingPass
language: rust
---

# ErrorHandlingPass

Transforms nullable types, error integer codes (-1 / errno), and exceptions

## Signature

```rust
pub struct ErrorHandlingPass
```

## Visibility

- `pub`

## Docstring

Transforms nullable types, error integer codes (-1 / errno), and exceptions
into idiomatic Rust `Result<T, E>` and `Option<T>` types.

## Source
Lines 6–6 in `crates/forge-rust/src/refactor/error_handling.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [error_handling](/crates/forge-rust/src/refactor/error_handling.md) |
