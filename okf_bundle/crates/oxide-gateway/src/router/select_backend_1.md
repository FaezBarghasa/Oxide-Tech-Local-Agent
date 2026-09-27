---
okf_version: "0.2"
type: Function
title: select_backend
description: Probe the network concurrently and return the best available coder backend with minimal latency.
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/select_backend_1
language: rust
---

# select_backend

Probe the network concurrently and return the best available coder backend with minimal latency.

## Signature

```rust
pub fn select_backend(&self, primary_url: &str, secondary_url: &str) -> CoderBackend
```

## Visibility

- `pub`

## Docstring

Probe the network concurrently and return the best available coder backend with minimal latency.

## Source
Lines 88–112 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
