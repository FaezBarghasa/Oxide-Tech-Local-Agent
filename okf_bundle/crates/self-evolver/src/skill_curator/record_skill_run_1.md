---
okf_version: "0.2"
type: Function
title: record_skill_run
description: Record the execution of a skill trajectory in SurrealDB
resource: crates/self-evolver/src/skill_curator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/skill_curator/record_skill_run_1
language: rust
---

# record_skill_run

Record the execution of a skill trajectory in SurrealDB

## Signature

```rust
pub fn record_skill_run(
        &self,
        skill_name: &str,
        domain: &str,
        success: bool,
        error_log: Option<String>,
    ) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Record the execution of a skill trajectory in SurrealDB

## Source
Lines 43–104 in `crates/self-evolver/src/skill_curator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_curator](/crates/self-evolver/src/skill_curator.md) |
