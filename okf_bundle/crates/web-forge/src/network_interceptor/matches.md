---
okf_version: "0.2"
type: Function
title: matches
description: Check if an incoming request matches this rule.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/matches
language: rust
---

# matches

Check if an incoming request matches this rule.

## Signature

```rust
impl MockRule { pub fn matches(&self, url: &str, method: &str) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if an incoming request matches this rule.

## Source
Lines 28–44 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
