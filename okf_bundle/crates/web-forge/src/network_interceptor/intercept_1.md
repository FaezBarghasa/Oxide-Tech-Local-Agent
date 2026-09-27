---
okf_version: "0.2"
type: Function
title: intercept
description: Evaluate an intercepted request against configured rules and record history.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/intercept_1
language: rust
---

# intercept

Evaluate an intercepted request against configured rules and record history.

## Signature

```rust
pub fn intercept(&mut self, request: InterceptedRequest) -> InterceptDecision
```

## Visibility

- `pub`

## Docstring

Evaluate an intercepted request against configured rules and record history.

## Source
Lines 148–158 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
