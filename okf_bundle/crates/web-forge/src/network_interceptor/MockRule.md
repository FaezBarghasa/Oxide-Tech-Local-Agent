---
okf_version: "0.2"
type: Class
title: MockRule
description: Rule for matching intercepted network requests.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/MockRule
language: rust
---

# MockRule

Rule for matching intercepted network requests.

## Signature

```rust
pub struct MockRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rule for matching intercepted network requests.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `id`
- `url_pattern`
- `method`

## Source
Lines 7–11 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
