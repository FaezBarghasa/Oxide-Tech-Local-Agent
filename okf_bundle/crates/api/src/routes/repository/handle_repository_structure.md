---
okf_version: "0.2"
type: Function
title: handle_repository_structure
description: "[get(\"/api/repository/structure\")]"
resource: crates/api/src/routes/repository.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/repository/handle_repository_structure
language: rust
---

# handle_repository_structure

[get("/api/repository/structure")]

## Signature

```rust
pub fn handle_repository_structure(
    query: web::Query<RepoStructureRequest>,
) -> impl Responder
```

## Decorators

- `get("/api/repository/structure")`

## Visibility

- `pub`

## Docstring

[get("/api/repository/structure")]

## Source
Lines 99–114 in `crates/api/src/routes/repository.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [repository](/crates/api/src/routes/repository.md) |
| calls | [run_repository_structure](/crates/api/src/routes/repository/run_repository_structure.md) |
