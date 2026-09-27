---
okf_version: "0.2"
type: Class
title: NamingPass
description: "Normalizes identifier names to idiomatic Rust conventions:"
resource: crates/forge-rust/src/refactor/naming.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T06:54:02Z"
concept_id: crates/forge-rust/src/refactor/naming/NamingPass
language: rust
---

# NamingPass

Normalizes identifier names to idiomatic Rust conventions:

## Signature

```rust
pub struct NamingPass
```

## Visibility

- `pub`

## Docstring

Normalizes identifier names to idiomatic Rust conventions:
- Functions & variables: snake_case
- Structs, Enums, Traits: PascalCase
- Constants: SCREAMING_SNAKE_CASE

## Source
Lines 8–8 in `crates/forge-rust/src/refactor/naming.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [naming](/crates/forge-rust/src/refactor/naming.md) |
