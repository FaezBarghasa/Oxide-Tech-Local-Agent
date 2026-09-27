---
okf_version: "0.2"
type: Function
title: is_reachable
description: "Returns `true` if the endpoint is reachable within `timeout_ms`."
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/is_reachable_1
language: rust
---

# is_reachable

Returns `true` if the endpoint is reachable within `timeout_ms`.

## Signature

```rust
pub fn is_reachable(&self, base_url: &str, timeout_ms: u64) -> bool
```

## Visibility

- `pub`

## Docstring

Returns `true` if the endpoint is reachable within `timeout_ms`.

## Source
Lines 55–61 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
