---
okf_version: "0.2"
type: Function
title: run
resource: crates/forge-rust/src/refactor/naming.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T06:54:02Z"
concept_id: crates/forge-rust/src/refactor/naming/run_1
language: rust
---

# run

## Signature

```rust
fn run(&self, module: &mut UirModule)
```

## Source
Lines 11–53 in `crates/forge-rust/src/refactor/naming.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [naming](/crates/forge-rust/src/refactor/naming.md) |
| calls | [to_pascal_case](/crates/forge-rust/src/refactor/naming/to_pascal_case.md) |
| calls | [to_snake_case](/crates/forge-rust/src/refactor/naming/to_snake_case.md) |
| calls | [to_screaming_snake_case](/crates/forge-rust/src/refactor/naming/to_screaming_snake_case.md) |
