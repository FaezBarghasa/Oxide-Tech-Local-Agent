---
okf_version: "0.2"
type: Function
title: get_component_by_ref
resource: crates/surrealdb-service/src/queries/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/component/get_component_by_ref
language: rust
---

# get_component_by_ref

## Signature

```rust
pub fn get_component_by_ref(
    client: &SurrealClient,
    ref_des: &str,
) -> Result<Option<Component>, Error>
```

## Visibility

- `pub`

## Source
Lines 24–35 in `crates/surrealdb-service/src/queries/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/surrealdb-service/src/queries/component.md) |
