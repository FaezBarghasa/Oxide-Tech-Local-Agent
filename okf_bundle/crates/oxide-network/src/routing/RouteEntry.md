---
okf_version: "0.2"
type: Class
title: RouteEntry
description: A discrete route table entry
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing/RouteEntry
language: rust
---

# RouteEntry

A discrete route table entry

## Signature

```rust
pub struct RouteEntry
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A discrete route table entry
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `prefix`
- `target`
- `metric`
- `pmtu`

## Source
Lines 33–38 in `crates/oxide-network/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-network/src/routing.md) |
