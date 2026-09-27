---
okf_version: "0.2"
type: Function
title: get_project
resource: crates/surrealdb-service/src/queries/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/project/get_project
language: rust
---

# get_project

## Signature

```rust
pub fn get_project(client: &SurrealClient, name: &str) -> Result<Option<Project>, Error>
```

## Visibility

- `pub`

## Source
Lines 24–32 in `crates/surrealdb-service/src/queries/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/surrealdb-service/src/queries/project.md) |
