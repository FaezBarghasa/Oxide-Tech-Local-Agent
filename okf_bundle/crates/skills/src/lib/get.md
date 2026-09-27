---
okf_version: "0.2"
type: Function
title: get
resource: crates/skills/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:skills"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/skills/src/lib/get
language: rust
---

# get

## Signature

```rust
impl SkillRegistry { pub fn get(&self, name: &str) -> Option<&SkillInfo> }
```

## Visibility

- `pub`

## Source
Lines 29–31 in `crates/skills/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/skills/src/lib.md) |
