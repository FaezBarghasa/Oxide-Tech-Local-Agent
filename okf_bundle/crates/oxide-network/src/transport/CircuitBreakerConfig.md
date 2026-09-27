---
okf_version: "0.2"
type: Class
title: CircuitBreakerConfig
description: Transport Circuit Breaker preventing connection cascades to dead peers
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/CircuitBreakerConfig
language: rust
---

# CircuitBreakerConfig

Transport Circuit Breaker preventing connection cascades to dead peers

## Signature

```rust
pub struct CircuitBreakerConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Transport Circuit Breaker preventing connection cascades to dead peers
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `failure_threshold`
- `recovery_timeout_secs`

## Source
Lines 166–169 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
