---
okf_version: "0.2"
type: Function
title: get_net_connections
resource: crates/surrealdb-service/src/queries/netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:11Z"
concept_id: crates/surrealdb-service/src/queries/netlist/get_net_connections
language: rust
---

# get_net_connections

## Signature

```rust
pub fn get_net_connections(
    client: &SurrealClient,
    net_name: &str,
) -> Result<Vec<Component>, Error>
```

## Visibility

- `pub`

## Source
Lines 5–15 in `crates/surrealdb-service/src/queries/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/surrealdb-service/src/queries/netlist.md) |
