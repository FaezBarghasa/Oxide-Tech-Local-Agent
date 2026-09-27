---
okf_version: "0.2"
type: Function
title: get_compilation_history
resource: crates/surrealdb-service/src/queries/compilation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/compilation/get_compilation_history
language: rust
---

# get_compilation_history

## Signature

```rust
pub fn get_compilation_history(
    client: &SurrealClient,
    project_id: RecordId,
) -> Result<Vec<CompilationRecord>, Error>
```

## Visibility

- `pub`

## Source
Lines 28–39 in `crates/surrealdb-service/src/queries/compilation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compilation](/crates/surrealdb-service/src/queries/compilation.md) |
