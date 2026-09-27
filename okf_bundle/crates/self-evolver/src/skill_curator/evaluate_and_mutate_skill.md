---
okf_version: "0.2"
type: Function
title: evaluate_and_mutate_skill
description: Alias for SkillOpt evaluation and mutation
resource: crates/self-evolver/src/skill_curator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/skill_curator/evaluate_and_mutate_skill
language: rust
---

# evaluate_and_mutate_skill

Alias for SkillOpt evaluation and mutation

## Signature

```rust
impl SkillCurator<C> { pub fn evaluate_and_mutate_skill(&self, skill_name: &str) -> Result<()> }
```

## Type Parameters

- `C: Connection`

## Visibility

- `pub`

## Docstring

Alias for SkillOpt evaluation and mutation

## Source
Lines 134–137 in `crates/self-evolver/src/skill_curator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_curator](/crates/self-evolver/src/skill_curator.md) |
