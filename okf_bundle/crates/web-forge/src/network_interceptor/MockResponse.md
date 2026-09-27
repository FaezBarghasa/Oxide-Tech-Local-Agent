---
okf_version: "0.2"
type: Class
title: MockResponse
description: Simulated response for causal mocking and fault injection.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/MockResponse
language: rust
---

# MockResponse

Simulated response for causal mocking and fault injection.

## Signature

```rust
pub struct MockResponse
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Simulated response for causal mocking and fault injection.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `status_code`
- `headers`
- `body`
- `delay_ms`

## Source
Lines 49–54 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
