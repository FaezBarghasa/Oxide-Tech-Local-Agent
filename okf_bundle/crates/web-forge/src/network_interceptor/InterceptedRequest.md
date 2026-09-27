---
okf_version: "0.2"
type: Class
title: InterceptedRequest
description: Metadata of an intercepted HTTP request.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/InterceptedRequest
language: rust
---

# InterceptedRequest

Metadata of an intercepted HTTP request.

## Signature

```rust
pub struct InterceptedRequest
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Metadata of an intercepted HTTP request.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `request_id`
- `url`
- `method`
- `headers`
- `body`
- `timestamp`

## Source
Lines 93–100 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
