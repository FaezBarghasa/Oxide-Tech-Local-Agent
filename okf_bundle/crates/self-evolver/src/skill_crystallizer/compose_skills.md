---
okf_version: "0.2"
type: Function
title: compose_skills
description: Compose multiple crystallized skills into a composite higher-order workflow
resource: crates/self-evolver/src/skill_crystallizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T12:54:04Z"
concept_id: crates/self-evolver/src/skill_crystallizer/compose_skills
language: rust
---

# compose_skills

Compose multiple crystallized skills into a composite higher-order workflow

## Signature

```rust
impl SkillCrystallizer { pub fn compose_skills(
        &self,
        name: &str,
        description: &str,
        skills: &[&CrystallizedSkill],
    ) -> CrystallizedSkill }
```

## Visibility

- `pub`

## Docstring

Compose multiple crystallized skills into a composite higher-order workflow

## Source
Lines 64–91 in `crates/self-evolver/src/skill_crystallizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_crystallizer](/crates/self-evolver/src/skill_crystallizer.md) |
