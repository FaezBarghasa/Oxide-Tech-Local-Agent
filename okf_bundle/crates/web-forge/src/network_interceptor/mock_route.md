---
okf_version: "0.2"
type: Function
title: mock_route
description: Convenience helper to mock a route with status and JSON payload.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/mock_route
language: rust
---

# mock_route

Convenience helper to mock a route with status and JSON payload.

## Signature

```rust
impl NetworkInterceptor { pub fn mock_route(&mut self, url_pattern: &str, status: u16, body_json: &str) }
```

## Visibility

- `pub`

## Docstring

Convenience helper to mock a route with status and JSON payload.

## Source
Lines 141–145 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
