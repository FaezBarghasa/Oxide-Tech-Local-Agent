---
okf_version: "0.2"
type: Function
title: history
description: Get historical log of all intercepted requests.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/history_1
language: rust
---

# history

Get historical log of all intercepted requests.

## Signature

```rust
pub fn history(&self) -> &[InterceptedRequest]
```

## Visibility

- `pub`

## Docstring

Get historical log of all intercepted requests.

## Source
Lines 161–163 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
