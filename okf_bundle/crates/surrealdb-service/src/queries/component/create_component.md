---
okf_version: "0.2"
type: Function
title: create_component
resource: crates/surrealdb-service/src/queries/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/component/create_component
language: rust
---

# create_component

## Signature

```rust
pub fn create_component(
    client: &SurrealClient,
    ref_des: &str,
    value: &str,
    footprint: &str,
) -> Result<Component, Error>
```

## Visibility

- `pub`

## Source
Lines 5–22 in `crates/surrealdb-service/src/queries/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/surrealdb-service/src/queries/component.md) |
