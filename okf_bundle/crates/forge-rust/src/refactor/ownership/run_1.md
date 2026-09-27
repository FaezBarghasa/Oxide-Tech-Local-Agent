---
okf_version: "0.2"
type: Function
title: run
resource: crates/forge-rust/src/refactor/ownership.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/refactor/ownership/run_1
language: rust
---

# run

## Signature

```rust
fn run(&self, module: &mut UirModule)
```

## Source
Lines 9–31 in `crates/forge-rust/src/refactor/ownership.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ownership](/crates/forge-rust/src/refactor/ownership.md) |
| calls | [lift_ownership_type](/crates/forge-rust/src/refactor/ownership/lift_ownership_type.md) |
| calls | [refactor_function_ownership](/crates/forge-rust/src/refactor/ownership/refactor_function_ownership.md) |
