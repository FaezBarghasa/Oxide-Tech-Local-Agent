---
okf_version: "0.2"
type: Function
title: run_cargo_check
resource: crates/api/src/routes/cargo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/cargo/run_cargo_check
language: rust
---

# run_cargo_check

## Signature

```rust
pub fn run_cargo_check(req: CargoRequest) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Source
Lines 9–24 in `crates/api/src/routes/cargo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cargo](/crates/api/src/routes/cargo.md) |
| called_by | [handle_cargo_check](/crates/api/src/routes/cargo/handle_cargo_check.md) |
