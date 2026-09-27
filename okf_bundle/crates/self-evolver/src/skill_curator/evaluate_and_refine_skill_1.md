---
okf_version: "0.2"
type: Function
title: evaluate_and_refine_skill
description: "Evaluates a skill's historical performance in SurrealDB and mutates it if degraded (failures >= 2)"
resource: crates/self-evolver/src/skill_curator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/skill_curator/evaluate_and_refine_skill_1
language: rust
---

# evaluate_and_refine_skill

Evaluates a skill's historical performance in SurrealDB and mutates it if degraded (failures >= 2)

## Signature

```rust
pub fn evaluate_and_refine_skill(&self, skill_name: &str) -> Result<bool>
```

## Visibility

- `pub`

## Docstring

Evaluates a skill's historical performance in SurrealDB and mutates it if degraded (failures >= 2)

## Source
Lines 107–131 in `crates/self-evolver/src/skill_curator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_curator](/crates/self-evolver/src/skill_curator.md) |
