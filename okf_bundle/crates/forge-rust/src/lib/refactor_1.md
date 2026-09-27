---
okf_version: "0.2"
type: Function
title: refactor
description: Refactors foreign source code into idiomatic Rust.
resource: crates/forge-rust/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/lib/refactor_1
language: rust
---

# refactor

Refactors foreign source code into idiomatic Rust.

## Signature

```rust
pub fn refactor(
        source: &str,
        config: RefactorConfig,
    ) -> Result<RefactorResult, ForgeRustError>
```

## Visibility

- `pub`

## Docstring

Refactors foreign source code into idiomatic Rust.

## Source
Lines 66–104 in `crates/forge-rust/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/forge-rust/src/lib.md) |
