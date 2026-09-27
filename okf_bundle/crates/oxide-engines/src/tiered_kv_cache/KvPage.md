---
okf_version: "0.2"
type: Class
title: KvPage
description: A discrete page of Key-Value tensor blocks
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/KvPage
language: rust
---

# KvPage

A discrete page of Key-Value tensor blocks

## Signature

```rust
pub struct KvPage
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A discrete page of Key-Value tensor blocks
[derive(Debug, Clone)]

## Methods

- `page_id`
- `token_start`
- `token_count`
- `tier`
- `host_buffer`
- `device_ptr`

## Source
Lines 76–85 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
