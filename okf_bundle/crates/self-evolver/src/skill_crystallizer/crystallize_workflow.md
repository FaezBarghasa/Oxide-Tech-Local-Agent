---
okf_version: "0.2"
type: Function
title: crystallize_workflow
description: Distill a successful multi-step task execution into a crystallized skill
resource: crates/self-evolver/src/skill_crystallizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T12:54:04Z"
concept_id: crates/self-evolver/src/skill_crystallizer/crystallize_workflow
language: rust
---

# crystallize_workflow

Distill a successful multi-step task execution into a crystallized skill

## Signature

```rust
impl SkillCrystallizer { pub fn crystallize_workflow(&self, skill: &CrystallizedSkill) -> Result<PathBuf, String> }
```

## Visibility

- `pub`

## Docstring

Distill a successful multi-step task execution into a crystallized skill

## Source
Lines 46–61 in `crates/self-evolver/src/skill_crystallizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_crystallizer](/crates/self-evolver/src/skill_crystallizer.md) |
