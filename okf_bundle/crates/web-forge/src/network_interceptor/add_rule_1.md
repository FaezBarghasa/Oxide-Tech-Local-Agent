---
okf_version: "0.2"
type: Function
title: add_rule
description: Add a mock rule and its corresponding mock response.
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/add_rule_1
language: rust
---

# add_rule

Add a mock rule and its corresponding mock response.

## Signature

```rust
pub fn add_rule(&mut self, rule: MockRule, response: MockResponse)
```

## Visibility

- `pub`

## Docstring

Add a mock rule and its corresponding mock response.

## Source
Lines 136–138 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
