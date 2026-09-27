---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-state/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:00:09Z"
concept_id: crates/oxide-state/src/lib/new
language: rust
---

# new

## Signature

```rust
impl AppState { pub fn new(db_endpoint: &str) -> Result<Arc<Self>, surrealdb::Error> }
```

## Visibility

- `pub`

## Source
Lines 39–79 in `crates/oxide-state/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-state/src/lib.md) |
