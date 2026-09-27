---
okf_version: "0.2"
type: Function
title: scaffold
resource: crates/forge-rust/src/scaffold.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/scaffold/scaffold_1
language: rust
---

# scaffold

## Signature

```rust
pub fn scaffold(
        crate_name: &str,
        emitted_code: &str,
        dependencies: &[String],
        is_binary: bool,
    ) -> HashMap<String, String>
```

## Visibility

- `pub`

## Source
Lines 7–61 in `crates/forge-rust/src/scaffold.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scaffold](/crates/forge-rust/src/scaffold.md) |
