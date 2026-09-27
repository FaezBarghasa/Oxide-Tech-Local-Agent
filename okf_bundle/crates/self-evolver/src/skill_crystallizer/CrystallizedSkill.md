---
okf_version: "0.2"
type: Class
title: CrystallizedSkill
description: Crystallized Skill metadata and recipe format (compatible with Antigravity / Agent Skill spec)
resource: crates/self-evolver/src/skill_crystallizer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T12:54:04Z"
concept_id: crates/self-evolver/src/skill_crystallizer/CrystallizedSkill
language: rust
---

# CrystallizedSkill

Crystallized Skill metadata and recipe format (compatible with Antigravity / Agent Skill spec)

## Signature

```rust
pub struct CrystallizedSkill
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Crystallized Skill metadata and recipe format (compatible with Antigravity / Agent Skill spec)
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `name`
- `description`
- `tags`
- `prompt_template`
- `step_sequence`
- `source_task_id`

## Source
Lines 7–14 in `crates/self-evolver/src/skill_crystallizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_crystallizer](/crates/self-evolver/src/skill_crystallizer.md) |
