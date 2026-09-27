---
okf_version: "0.2"
type: Function
title: list_projects
resource: crates/surrealdb-service/src/queries/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/project/list_projects
language: rust
---

# list_projects

## Signature

```rust
pub fn list_projects(client: &SurrealClient) -> Result<Vec<Project>, Error>
```

## Visibility

- `pub`

## Source
Lines 34–38 in `crates/surrealdb-service/src/queries/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/surrealdb-service/src/queries/project.md) |
