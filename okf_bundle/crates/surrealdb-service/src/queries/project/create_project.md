---
okf_version: "0.2"
type: Function
title: create_project
resource: crates/surrealdb-service/src/queries/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/project/create_project
language: rust
---

# create_project

## Signature

```rust
pub fn create_project(
    client: &SurrealClient,
    name: &str,
    mcu_type: &str,
    target_triple: Option<String>,
) -> Result<Project, Error>
```

## Visibility

- `pub`

## Source
Lines 5–22 in `crates/surrealdb-service/src/queries/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/surrealdb-service/src/queries/project.md) |
