---
okf_version: "0.2"
type: Function
title: list_components
resource: crates/surrealdb-service/src/queries/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/component/list_components
language: rust
---

# list_components

## Signature

```rust
pub fn list_components(client: &SurrealClient) -> Result<Vec<Component>, Error>
```

## Visibility

- `pub`

## Source
Lines 37–41 in `crates/surrealdb-service/src/queries/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/surrealdb-service/src/queries/component.md) |
