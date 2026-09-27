---
okf_version: "0.2"
type: Function
title: client_for
description: "Get the `LlmRouterClient` reference for the given backend."
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/client_for_1
language: rust
---

# client_for

Get the `LlmRouterClient` reference for the given backend.

## Signature

```rust
pub fn client_for(&self, backend: CoderBackend) -> &LlmRouterClient
```

## Visibility

- `pub`

## Docstring

Get the `LlmRouterClient` reference for the given backend.

## Source
Lines 115–121 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
