---
okf_version: "0.2"
type: Function
title: is_satisfying
description: "Returns `true` if the score meets the configured threshold."
resource: crates/oxide-gateway/src/quality.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/quality/is_satisfying_1
language: rust
---

# is_satisfying

Returns `true` if the score meets the configured threshold.

## Signature

```rust
pub fn is_satisfying(&self, stderr: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Returns `true` if the score meets the configured threshold.

## Source
Lines 48–51 in `crates/oxide-gateway/src/quality.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [quality](/crates/oxide-gateway/src/quality.md) |
