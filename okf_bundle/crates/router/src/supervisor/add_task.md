---
okf_version: "0.2"
type: Function
title: add_task
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/add_task
language: rust
---

# add_task

## Signature

```rust
impl TaskDag { pub fn add_task(
        &mut self,
        id: &str,
        title: &str,
        role: SubAgentRole,
        description: &str,
        dependencies: Vec<&str>,
    ) }
```

## Visibility

- `pub`

## Source
Lines 120–142 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
