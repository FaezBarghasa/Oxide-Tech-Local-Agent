---
okf_version: "0.2"
type: Function
title: new
resource: crates/self-evolver/src/skill_curator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/skill_curator/new
language: rust
---

# new

## Signature

```rust
impl SkillCurator<C> { pub fn new(db: Arc<Surreal<C>>, vllm_endpoint: &str, skills_base_dir: PathBuf) -> Self }
```

## Type Parameters

- `C: Connection`

## Visibility

- `pub`

## Source
Lines 28–35 in `crates/self-evolver/src/skill_curator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skill_curator](/crates/self-evolver/src/skill_curator.md) |
