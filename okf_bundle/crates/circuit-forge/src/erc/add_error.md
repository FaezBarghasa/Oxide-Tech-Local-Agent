---
okf_version: "0.2"
type: Function
title: add_error
description: Add an error to the report.
resource: crates/circuit-forge/src/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:31:54Z"
concept_id: crates/circuit-forge/src/erc/add_error
language: rust
---

# add_error

Add an error to the report.

## Signature

```rust
impl ErcReport { pub fn add_error(
        &mut self,
        rule_id: &str,
        target_ref: Option<&str>,
        message: impl Into<String>,
    ) }
```

## Visibility

- `pub`

## Docstring

Add an error to the report.

## Source
Lines 36–50 in `crates/circuit-forge/src/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/circuit-forge/src/erc.md) |
