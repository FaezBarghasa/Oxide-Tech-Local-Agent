---
okf_version: "0.2"
type: Function
title: poll_and_update
description: "Poll system metrics, updating hysteresis circuit breaker"
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/poll_and_update
language: rust
---

# poll_and_update

Poll system metrics, updating hysteresis circuit breaker

## Signature

```rust
impl ResourceGater { pub fn poll_and_update(&self, free_vram_mb: Option<u64>) -> GateStatus }
```

## Visibility

- `pub`

## Docstring

Poll system metrics, updating hysteresis circuit breaker

## Source
Lines 82–114 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
