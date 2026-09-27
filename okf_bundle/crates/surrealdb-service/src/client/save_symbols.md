---
okf_version: "0.2"
type: Function
title: save_symbols
resource: crates/surrealdb-service/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/client/save_symbols
language: rust
---

# save_symbols

## Signature

```rust
impl SurrealClient { pub fn save_symbols(&self, symbols: &[ParsedSymbol]) -> Result<(), surrealdb::Error> }
```

## Visibility

- `pub`

## Source
Lines 26–99 in `crates/surrealdb-service/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/surrealdb-service/src/client.md) |
