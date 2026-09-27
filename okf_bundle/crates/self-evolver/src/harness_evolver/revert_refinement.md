---
okf_version: "0.2"
type: Function
title: revert_refinement
description: Reverts a rule by its rule_id
resource: crates/self-evolver/src/harness_evolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/harness_evolver/revert_refinement
language: rust
---

# revert_refinement

Reverts a rule by its rule_id

## Signature

```rust
impl HarnessEvolver { pub fn revert_refinement(&self, rule_id: &str) -> Result<bool> }
```

## Visibility

- `pub`

## Docstring

Reverts a rule by its rule_id

## Source
Lines 117–148 in `crates/self-evolver/src/harness_evolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness_evolver](/crates/self-evolver/src/harness_evolver.md) |
