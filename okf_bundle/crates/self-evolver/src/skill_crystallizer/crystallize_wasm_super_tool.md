---
okf_version: "0.2"
type: Function
title: crystallize_wasm_super_tool
description: Crystallize a high-frequency composite skill into a high-performance Wasm tool manifest
resource: crates/self-evolver/src/skill_crystallizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T12:54:04Z"
concept_id: crates/self-evolver/src/skill_crystallizer/crystallize_wasm_super_tool
language: rust
---

# crystallize_wasm_super_tool

Crystallize a high-frequency composite skill into a high-performance Wasm tool manifest

## Signature

```rust
impl SkillCrystallizer { pub fn crystallize_wasm_super_tool(
        &self,
        skill: &CrystallizedSkill,
        wasm_bytes: &[u8],
    ) -> Result<PathBuf, String> }
```

## Visibility

- `pub`

## Docstring

Crystallize a high-frequency composite skill into a high-performance Wasm tool manifest

## Source
Lines 94–128 in `crates/self-evolver/src/skill_crystallizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_crystallizer](/crates/self-evolver/src/skill_crystallizer.md) |
