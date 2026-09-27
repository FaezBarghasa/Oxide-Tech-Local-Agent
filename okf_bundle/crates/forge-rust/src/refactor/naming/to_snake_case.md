---
okf_version: "0.2"
type: Function
title: to_snake_case
resource: crates/forge-rust/src/refactor/naming.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T06:54:02Z"
concept_id: crates/forge-rust/src/refactor/naming/to_snake_case
language: rust
---

# to_snake_case

## Signature

```rust
pub fn to_snake_case(s: &str) -> String
```

## Visibility

- `pub`

## Source
Lines 56–90 in `crates/forge-rust/src/refactor/naming.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [naming](/crates/forge-rust/src/refactor/naming.md) |
| called_by | [run](/crates/forge-rust/src/refactor/naming/run.md) |
| called_by | [to_screaming_snake_case](/crates/forge-rust/src/refactor/naming/to_screaming_snake_case.md) |
